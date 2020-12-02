use crate::claim::claim_from_str::claim_from_str;
use crate::claim::{claim_is_global, Claim};

pub fn claim_direct_descendant_str<'a>(claim: Claim<'a>, query: &str) -> Option<&'a str> {
    let parse_result = claim_from_str(query);
    if let Ok(parsed) = parse_result {
        claim_direct_descendant(claim, parsed)
    } else {
        None
    }
}

pub fn claim_direct_descendant<'a>(claim: Claim<'a>, query: Claim) -> Option<&'a str> {
    if claim.0 != query.0 || claim_is_global(claim) {
        return None;
    }

    if claim_is_global(query) {
        return match claim.1.find(".") {
            None => Some(claim.1),
            Some(idx) => Some(&claim.1[..idx]),
        };
    }

    if !claim.1.starts_with(format!("{}.", query.1).as_str()) {
        return None;
    }

    let len = query.1.len() + 1;
    let rest = &claim.1[len..];

    return match rest.find(".") {
        None => Some(rest),
        Some(idx) => Some(&rest[..idx]),
    };
}

pub fn claim_direct_child_str<'a>(claim: Claim<'a>, query: &str) -> Option<&'a str> {
    let parse_result = claim_from_str(query);
    if let Ok(parsed) = parse_result {
        claim_direct_child(claim, parsed)
    } else {
        None
    }
}

pub fn claim_direct_child<'a>(claim: Claim<'a>, query: Claim) -> Option<&'a str> {
    if claim.0 != query.0 || claim_is_global(claim) {
        return None;
    }

    if claim_is_global(query) {
        return if claim.1.contains(".") {
            None
        } else {
            Some(claim.1)
        };
    }

    if !claim.1.starts_with(format!("{}.", query.1).as_str()) {
        return None;
    }

    let len = query.1.len() + 1;
    let rest = &claim.1[len..];

    return if rest.contains(".") { None } else { Some(rest) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direct_descendant_valid_global() {
        let claim: Claim = ("read", "paco");
        assert_eq!(claim_direct_descendant_str(claim, "read:*"), Some("paco"));
    }

    #[test]
    fn test_direct_descendant_valid() {
        let claim: Claim = ("read", "something.or.other");
        assert_eq!(
            claim_direct_descendant_str(claim, "read:*"),
            Some("something")
        );
        assert_eq!(
            claim_direct_descendant_str(claim, "read:something"),
            Some("or")
        );
        assert_eq!(
            claim_direct_descendant_str(claim, "read:something.or"),
            Some("other")
        );
    }

    #[test]
    fn test_direct_descendant_global() {
        let claim: Claim = ("read", "");
        assert_eq!(claim_direct_descendant_str(claim, "read:*"), None);
        assert_eq!(claim_direct_descendant_str(claim, "read:something"), None);
        assert_eq!(claim_direct_descendant_str(claim, "admin:*"), None);
        assert_eq!(claim_direct_descendant_str(claim, "admin:something"), None);
    }

    #[test]
    fn test_direct_descendant_invalid() {
        let claim: Claim = ("read", "something.or.other");
        assert_eq!(claim_direct_descendant_str(claim, "read:another"), None);
        assert_eq!(claim_direct_descendant_str(claim, "admin:something"), None);
        assert_eq!(
            claim_direct_descendant_str(claim, "read:something.or.other"),
            None
        );
    }

    #[test]
    fn test_direct_child_valid_global() {
        let claim: Claim = ("read", "paco");
        assert_eq!(claim_direct_child_str(claim, "read:*"), Some("paco"));
    }

    #[test]
    fn test_direct_child_valid() {
        let claim: Claim = ("read", "something.or.other");
        assert_eq!(claim_direct_child_str(claim, "read:*"), None);
        assert_eq!(claim_direct_child_str(claim, "read:something"), None);
        assert_eq!(
            claim_direct_child_str(claim, "read:something.or"),
            Some("other")
        );
    }

    #[test]
    fn test_direct_child_global() {
        let claim: Claim = ("read", "");
        assert_eq!(claim_direct_child_str(claim, "read:*"), None);
        assert_eq!(claim_direct_child_str(claim, "read:something"), None);
        assert_eq!(claim_direct_child_str(claim, "admin:*"), None);
        assert_eq!(claim_direct_child_str(claim, "admin:something"), None);
    }

    #[test]
    fn test_direct_child_invalid() {
        let claim: Claim = ("read", "something.or.other");
        assert_eq!(claim_direct_child_str(claim, "read:another"), None);
        assert_eq!(claim_direct_child_str(claim, "admin:something"), None);
        assert_eq!(
            claim_direct_child_str(claim, "read:something.or.other"),
            None
        );
    }
}
