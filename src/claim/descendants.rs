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

pub fn claims_direct_children_str<'a, I>(claims: I, query: &str) -> Vec<&'a str>
where
    I: Iterator<Item = &'a Claim<'a>>,
{
    let parse_result = claim_from_str(query);
    if let Ok(parsed) = parse_result {
        claims_direct_children(claims, parsed)
    } else {
        Vec::new()
    }
}

pub fn claims_direct_children<'a, I>(claims: I, query: Claim) -> Vec<&'a str>
where
    I: Iterator<Item = &'a Claim<'a>>,
{
    let mut vec: Vec<&'a str> = claims
        .filter_map(|&c| claim_direct_child(c, query))
        .collect();

    vec.sort();
    vec.dedup();
    vec
}

pub fn claims_direct_descendants_str<'a, I>(claims: I, query: &str) -> Vec<&'a str>
where
    I: Iterator<Item = &'a Claim<'a>>,
{
    let parse_result = claim_from_str(query);
    if let Ok(parsed) = parse_result {
        claims_direct_descendants(claims, parsed)
    } else {
        Vec::new()
    }
}

pub fn claims_direct_descendants<'a, I>(claims: I, query: Claim) -> Vec<&'a str>
where
    I: Iterator<Item = &'a Claim<'a>>,
{
    let mut vec: Vec<&'a str> = claims
        .filter_map(|&c| claim_direct_descendant(c, query))
        .collect();

    vec.sort();
    vec.dedup();
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claims_direct_children_with_bad_query() {
        let claims = [("read", "paco"), ("read", "something")];
        let query = "adminasdasda";
        let expected: Vec<&str> = Vec::new();
        assert_eq!(claims_direct_children_str(claims.iter(), query), expected)
    }

    #[test]
    fn test_claims_direct_children_with_none() {
        let claims = [("read", "paco"), ("read", "something")];
        let query = "admin:whatever";
        let expected: Vec<&str> = Vec::new();
        assert_eq!(claims_direct_children_str(claims.iter(), query), expected)
    }

    #[test]
    fn test_claims_direct_children_with_some() {
        let claims = [
            ("read", "paco"),
            ("read", "paco"),
            ("read", "something"),
            ("admin", "blah"),
        ];
        let query = "read:*";
        let expected: Vec<&str> = ["paco", "something"].to_vec();
        assert_eq!(claims_direct_children_str(claims.iter(), query), expected)
    }

    #[test]
    fn test_claims_direct_descendants_with_bad_query() {
        let claims = [("read", "paco"), ("read", "something")];
        let query = "adminasdasda";
        let expected: Vec<&str> = Vec::new();
        assert_eq!(
            claims_direct_descendants_str(claims.iter(), query),
            expected
        )
    }

    #[test]
    fn test_claims_direct_descendants_with_none() {
        let claims = [("read", "paco"), ("read", "something")];
        let query = "admin:whatever";
        let expected: Vec<&str> = Vec::new();
        assert_eq!(
            claims_direct_descendants_str(claims.iter(), query),
            expected
        )
    }

    #[test]
    fn test_claims_direct_descendants_with_some() {
        let claims = [
            ("read", "paco.what"),
            ("read", "paco.and.something"),
            ("read", "paco.and.another"),
            ("read", "paco"),
            ("admin", "blah"),
        ];
        let query = "read:paco";
        let expected: Vec<&str> = ["and", "what"].to_vec();
        assert_eq!(
            claims_direct_descendants_str(claims.iter(), query),
            expected
        )
    }

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
