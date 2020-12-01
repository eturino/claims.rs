use crate::claim::claim_from_str::claim_from_str;
use crate::claim::Claim;

pub fn claim_is_global(claim: Claim) -> bool {
    claim.1.len() == 0
}

pub fn claim_check_str(claim: Claim, query: &str) -> bool {
    let parse_result = claim_from_str(query);
    if let Ok(parsed) = parse_result {
        claim_check(claim, parsed)
    } else {
        false
    }
}

pub fn claim_check(claim: Claim, query: Claim) -> bool {
    if claim.0 != query.0 {
        return false;
    }

    if claim_is_global(claim) {
        return true;
    }

    if claim_is_global(query) {
        return false;
    }

    if claim.1 == query.1 {
        return true;
    }

    query.1.starts_with(format!("{}.", claim.1).as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_valid(claim: Claim, query: &str) {
        let msg = format!("claim: '{:?}' with query '{}' should work", claim, query);
        assert!(claim_check_str(claim, query), msg)
    }

    fn check_invalid(claim: Claim, query: &str) {
        let msg = format!("claim: '{:?}' with query '{}' should fail", claim, query);
        assert!(!claim_check_str(claim, query), msg)
    }

    #[test]
    fn test_claim_is_global() {
        assert!(claim_is_global(("read", "")));
        assert!(!claim_is_global(("read", "paco")));
    }

    #[test]
    fn test_with_invalid_query() {
        assert!(!claim_check_str(("read", ""), "whatever-this-is"));
    }

    #[test]
    fn global_claim_with_valid() {
        let claim = ("read", "");

        let list = [
            "read:*",
            "read:something",
            "read:some-like_this.stuff-or_o.even-with-99",
        ];
        for x in list.iter() {
            check_valid(claim, x)
        }
    }

    #[test]
    fn global_claim_with_invalid() {
        let claim = ("read", "");
        let list = ["admin:*", "admin:something"];
        for x in list.iter() {
            check_invalid(claim, x)
        }
    }

    #[test]
    fn specific_claim_with_valid() {
        let claim = ("read", "something");
        let list = ["read:something", "read:something.else"];
        for x in list.iter() {
            check_valid(claim, x)
        }
    }

    #[test]
    fn specific_claim_with_invalid() {
        let claim = ("read", "something");
        let list = ["read:*", "admin:something", "admin:*", "admin:something"];
        for x in list.iter() {
            check_invalid(claim, x)
        }
    }
}
