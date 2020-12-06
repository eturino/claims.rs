use crate::claim::claim_from_str::claim_from_str;
use crate::claim::Claim;

pub fn claim_check_str(claim: &Claim, query: &str) -> bool {
    let parse_result = claim_from_str(query);
    if let Ok(parsed) = parse_result {
        claim_check(claim, &parsed)
    } else {
        false
    }
}

pub fn claim_exact_str(claim: &Claim, query: &str) -> bool {
    let parse_result = claim_from_str(query);
    if let Ok(parsed) = parse_result {
        claim_exact(claim, &parsed)
    } else {
        false
    }
}

pub fn claim_exact(claim: &Claim, query: &Claim) -> bool {
    claim.verb == query.verb && claim.subject == query.subject
}

pub fn claim_check(claim: &Claim, query: &Claim) -> bool {
    if claim.verb != query.verb {
        return false;
    }

    if claim.is_global() {
        return true;
    }

    if query.is_global() {
        return false;
    }

    if claim.subject == query.subject {
        return true;
    }

    query
        .subject
        .starts_with(format!("{}.", claim.subject).as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_valid(claim: &Claim, query: &str) {
        let msg = format!("claim: '{:?}' with query '{}' should work", claim, query);
        assert!(claim_check_str(claim, query), msg)
    }

    fn check_invalid(claim: &Claim, query: &str) {
        let msg = format!("claim: '{:?}' with query '{}' should fail", claim, query);
        assert!(!claim_check_str(claim, query), msg)
    }

    #[test]
    fn exact_with_invalid_query() {
        assert!(!claim_exact_str(
            &Claim::new("read", ""),
            "whatever-this-is"
        ));
    }

    #[test]
    fn check_with_invalid_query() {
        assert!(!claim_check_str(
            &Claim::new("read", ""),
            "whatever-this-is"
        ));
    }

    #[test]
    fn test_exact_with_same() {
        assert!(claim_check_str(&Claim::new("read", ""), "read:*"));
    }

    #[test]
    fn test_exact_with_different_verb() {
        assert!(!claim_check_str(&Claim::new("read", ""), "admin:*"));
        assert!(!claim_check_str(
            &Claim::new("read", "something"),
            "admin:something"
        ));
    }

    #[test]
    fn test_exact_with_different_subject() {
        assert!(!claim_exact_str(&Claim::new("read", ""), "read:blah"));
        assert!(!claim_exact_str(
            &Claim::new("read", "something"),
            "read:blah"
        ));
        assert!(!claim_exact_str(
            &Claim::new("read", "something"),
            "read:something.blah"
        ));
    }

    #[test]
    fn global_claim_with_valid() {
        let claim = Claim::new("read", "");

        let list = [
            "read:*",
            "read:something",
            "read:some-like_this.stuff-or_o.even-with-99",
        ];
        for x in list.iter() {
            check_valid(&claim, x)
        }
    }

    #[test]
    fn global_claim_with_invalid() {
        let claim = Claim::new("read", "");
        let list = ["admin:*", "admin:something"];
        for x in list.iter() {
            check_invalid(&claim, x)
        }
    }

    #[test]
    fn specific_claim_with_valid() {
        let claim = Claim::new("read", "something");
        let list = ["read:something", "read:something.else"];
        for x in list.iter() {
            check_valid(&claim, x)
        }
    }

    #[test]
    fn specific_claim_with_invalid() {
        let claim = Claim::new("read", "something");
        let list = ["read:*", "admin:something", "admin:*", "admin:something"];
        for x in list.iter() {
            check_invalid(&claim, x)
        }
    }
}
