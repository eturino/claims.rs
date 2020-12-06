use crate::claim::claim_from_str::claim_from_str;
use crate::claim::Claim;

pub fn claim_direct_descendant_str(claim: &Claim, query: &str) -> Option<String> {
    let parse_result = claim_from_str(query);
    if let Ok(parsed) = parse_result {
        claim_direct_descendant(claim, &parsed)
    } else {
        None
    }
}

pub fn claim_direct_descendant(claim: &Claim, query: &Claim) -> Option<String> {
    if claim.verb != query.verb || claim.is_global() {
        return None;
    }

    if query.is_global() {
        return match claim.subject.find(".") {
            None => Some(claim.subject.clone()),
            Some(idx) => Some(String::from(&claim.subject[..idx])),
        };
    }

    if !claim
        .subject
        .starts_with(format!("{}.", query.subject).as_str())
    {
        return None;
    }

    let len = query.subject.len() + 1;
    let rest = &claim.subject[len..];

    return match rest.find(".") {
        None => Some(String::from(rest)),
        Some(idx) => Some(String::from(&rest[..idx])),
    };
}

pub fn claim_direct_child_str(claim: &Claim, query: &str) -> Option<String> {
    let parse_result = claim_from_str(query);
    if let Ok(parsed) = parse_result {
        claim_direct_child(claim, &parsed)
    } else {
        None
    }
}

pub fn claim_direct_child(claim: &Claim, query: &Claim) -> Option<String> {
    if claim.verb != query.verb || claim.is_global() {
        return None;
    }

    if query.is_global() {
        return if claim.subject.contains(".") {
            None
        } else {
            Some(claim.subject.clone())
        };
    }

    if !claim
        .subject
        .starts_with(format!("{}.", query.subject).as_str())
    {
        return None;
    }

    let len = query.subject.len() + 1;
    let rest = &claim.subject[len..];

    return if rest.contains(".") {
        None
    } else {
        Some(String::from(rest))
    };
}

pub fn claims_direct_children_str<'a, I>(claims: I, query: &str) -> Vec<String>
where
    I: Iterator<Item = &'a Claim>,
{
    let parse_result = claim_from_str(query);
    if let Ok(parsed) = parse_result {
        claims_direct_children(claims, &parsed)
    } else {
        Vec::new()
    }
}

pub fn claims_direct_children<'a, I>(claims: I, query: &Claim) -> Vec<String>
where
    I: Iterator<Item = &'a Claim>,
{
    let mut vec: Vec<String> = claims
        .filter_map(|c| claim_direct_child(c, query))
        .collect();

    vec.sort();
    vec.dedup();
    vec
}

pub fn claims_direct_descendants_str<'a, I>(claims: I, query: &str) -> Vec<String>
where
    I: Iterator<Item = &'a Claim>,
{
    let parse_result = claim_from_str(query);
    if let Ok(parsed) = parse_result {
        claims_direct_descendants(claims, &parsed)
    } else {
        Vec::new()
    }
}

pub fn claims_direct_descendants<'a, I>(claims: I, query: &Claim) -> Vec<String>
where
    I: Iterator<Item = &'a Claim>,
{
    let mut vec: Vec<String> = claims
        .filter_map(|c| claim_direct_descendant(c, query))
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
        let claims = [Claim::new("read", "paco"), Claim::new("read", "something")];
        let query = "adminasdasda";
        let expected: Vec<String> = Vec::new();
        assert_eq!(claims_direct_children_str(claims.iter(), query), expected)
    }

    #[test]
    fn test_claims_direct_children_with_none() {
        let claims = [Claim::new("read", "paco"), Claim::new("read", "something")];
        let query = "admin:whatever";
        let expected: Vec<String> = Vec::new();
        assert_eq!(claims_direct_children_str(claims.iter(), query), expected)
    }

    #[test]
    fn test_claims_direct_children_with_some() {
        let claims = [
            Claim::new("read", "paco"),
            Claim::new("read", "paco"),
            Claim::new("read", "something"),
            Claim::new("admin", "blah"),
        ];
        let query = "read:*";
        let expected: Vec<String> = vec![String::from("paco"), String::from("something")];
        assert_eq!(claims_direct_children_str(claims.iter(), query), expected)
    }

    #[test]
    fn test_claims_direct_descendants_with_bad_query() {
        let claims = [Claim::new("read", "paco"), Claim::new("read", "something")];
        let query = "adminasdasda";
        let expected: Vec<&str> = Vec::new();
        assert_eq!(
            claims_direct_descendants_str(claims.iter(), query),
            expected
        )
    }

    #[test]
    fn test_claims_direct_descendants_with_none() {
        let claims = [Claim::new("read", "paco"), Claim::new("read", "something")];
        let query = "admin:whatever";
        let expected: Vec<String> = Vec::new();
        assert_eq!(
            claims_direct_descendants_str(claims.iter(), query),
            expected
        )
    }

    #[test]
    fn test_claims_direct_descendants_with_some() {
        let claims = [
            Claim::new("read", "paco.what"),
            Claim::new("read", "paco.and.something"),
            Claim::new("read", "paco.and.another"),
            Claim::new("read", "paco"),
            Claim::new("admin", "blah"),
        ];
        let query = "read:paco";
        let expected: Vec<String> = vec![String::from("and"), String::from("what")];
        assert_eq!(
            claims_direct_descendants_str(claims.iter(), query),
            expected
        )
    }

    #[test]
    fn test_direct_descendant_valid_global() {
        let claim = Claim::new("read", "paco");
        assert_eq!(
            claim_direct_descendant_str(&claim, "read:*"),
            Some(String::from("paco"))
        );
    }

    #[test]
    fn test_direct_descendant_valid() {
        let claim = Claim::new("read", "something.or.other");
        assert_eq!(
            claim_direct_descendant_str(&claim, "read:*"),
            Some(String::from("something"))
        );
        assert_eq!(
            claim_direct_descendant_str(&claim, "read:something"),
            Some(String::from("or"))
        );
        assert_eq!(
            claim_direct_descendant_str(&claim, "read:something.or"),
            Some(String::from("other"))
        );
    }

    #[test]
    fn test_direct_descendant_global() {
        let claim = Claim::new("read", "");
        assert_eq!(claim_direct_descendant_str(&claim, "read:*"), None);
        assert_eq!(claim_direct_descendant_str(&claim, "read:something"), None);
        assert_eq!(claim_direct_descendant_str(&claim, "admin:*"), None);
        assert_eq!(claim_direct_descendant_str(&claim, "admin:something"), None);
    }

    #[test]
    fn test_direct_descendant_invalid() {
        let claim = Claim::new("read", "something.or.other");
        assert_eq!(claim_direct_descendant_str(&claim, "read:another"), None);
        assert_eq!(claim_direct_descendant_str(&claim, "admin:something"), None);
        assert_eq!(
            claim_direct_descendant_str(&claim, "read:something.or.other"),
            None
        );
    }

    #[test]
    fn test_direct_child_valid_global() {
        let claim = Claim::new("read", "paco");
        assert_eq!(
            claim_direct_child_str(&claim, "read:*"),
            Some(String::from("paco"))
        );
    }

    #[test]
    fn test_direct_child_valid() {
        let claim = Claim::new("read", "something.or.other");
        assert_eq!(claim_direct_child_str(&claim, "read:*"), None);
        assert_eq!(claim_direct_child_str(&claim, "read:something"), None);
        assert_eq!(
            claim_direct_child_str(&claim, "read:something.or"),
            Some(String::from("other"))
        );
    }

    #[test]
    fn test_direct_child_global() {
        let claim = Claim::new("read", "");
        assert_eq!(claim_direct_child_str(&claim, "read:*"), None);
        assert_eq!(claim_direct_child_str(&claim, "read:something"), None);
        assert_eq!(claim_direct_child_str(&claim, "admin:*"), None);
        assert_eq!(claim_direct_child_str(&claim, "admin:something"), None);
    }

    #[test]
    fn test_direct_child_invalid() {
        let claim = Claim::new("read", "something.or.other");
        assert_eq!(claim_direct_child_str(&claim, "read:another"), None);
        assert_eq!(claim_direct_child_str(&claim, "admin:something"), None);
        assert_eq!(
            claim_direct_child_str(&claim, "read:something.or.other"),
            None
        );
    }
}
