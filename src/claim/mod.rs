mod check;
mod claim_from_str;
mod descendants;
mod is_valid_claim_str;

use crate::claim::check::{claim_check, claim_check_str, claim_exact, claim_exact_str};
use crate::claim::claim_from_str::claims_from_strs;
use crate::claim::descendants::{
    claim_direct_child, claim_direct_child_str, claim_direct_descendant,
    claim_direct_descendant_str,
};
use crate::error::Error;
use claim_from_str::claim_from_str;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Ordering;
use std::fmt;

lazy_static! {
    static ref CLAIM_REGEX: Regex =
        Regex::new(r"^([\w_\-]+):(\*|(\w[\w_.\-]*)(\.|\.\*)?)$").unwrap();
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Claim {
    verb: String,
    subject: String,
}

impl Claim {
    pub fn new(verb: &str, subject: &str) -> Self {
        Self {
            verb: String::from(verb),
            subject: String::from(subject),
        }
    }

    pub fn from_tuple(tuple: (&str, &str)) -> Self {
        Self {
            verb: String::from(tuple.0),
            subject: String::from(tuple.1),
        }
    }

    pub fn parse(string: &str) -> Result<Self, Error> {
        claim_from_str(string)
    }

    pub fn parse_list<'a, I>(claim_strs: I) -> Result<Vec<Claim>, Error>
    where
        I: Iterator<Item = &'a &'a str>,
    {
        claims_from_strs(claim_strs)
    }

    // INSTANCE METHODS

    pub fn cmp(&self, other: &Self) -> Ordering {
        let verb_comp = self.verb.cmp(&other.verb);
        if verb_comp == Ordering::Equal {
            self.subject.cmp(&other.subject)
        } else {
            verb_comp
        }
    }

    pub fn is_global(&self) -> bool {
        self.subject.len() == 0
    }

    pub fn is_exact(&self, query: &Claim) -> bool {
        claim_exact(&self, query)
    }

    pub fn is_exact_str(&self, query: &str) -> bool {
        claim_exact_str(&self, query)
    }

    pub fn check(&self, query: &Claim) -> bool {
        claim_check(&self, query)
    }

    pub fn check_str(&self, query: &str) -> bool {
        claim_check_str(&self, query)
    }

    pub fn direct_child(&self, query: &Claim) -> Option<String> {
        claim_direct_child(self, &query)
    }

    pub fn direct_child_str(&self, query: &str) -> Option<String> {
        claim_direct_child_str(self, &query)
    }

    pub fn direct_descendant(&self, query: &Claim) -> Option<String> {
        claim_direct_descendant(self, &query)
    }

    pub fn direct_descendant_str(&self, query: &str) -> Option<String> {
        claim_direct_descendant_str(self, &query)
    }
}

impl fmt::Display for Claim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sub = if self.subject == "" {
            "*"
        } else {
            &self.subject
        };
        write!(f, "{}:{}", self.verb, sub)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make(verb: &str, subject: &str) -> Claim {
        Claim::new(verb, subject)
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", make("read", "")), "read:*");
        assert_eq!(
            format!("{}", make("read", "something.else")),
            "read:something.else"
        );
    }

    #[test]
    fn test_claim_is_global() {
        assert!(make("read", "").is_global());
        assert!(!make("read", "paco").is_global());
    }

    #[test]
    fn test_parse() {
        assert_eq!(Claim::parse("read:*"), Ok(make("read", "")));
        assert_eq!(Claim::parse("read:a"), Ok(make("read", "a")));
        assert_eq!(Claim::parse("read:a."), Ok(make("read", "a")));
        assert_eq!(Claim::parse("read:a.*"), Ok(make("read", "a")));
        assert_eq!(
            Claim::parse("read:something.else"),
            Ok(make("read", "something.else"))
        );
        assert_eq!(
            Claim::parse("read:something.else.*"),
            Ok(make("read", "something.else"))
        );
        assert_eq!(
            Claim::parse("bad-stuff.*"),
            Err(Error::Syntax(
                "the given claim bad-stuff.* is not valid".to_string()
            ))
        );
    }

    #[test]
    fn test_parse_list() {
        assert_eq!(
            Claim::parse_list(["read:*", "read:*"].iter()),
            Ok(vec![make("read", "")])
        );
        assert_eq!(
            Claim::parse_list(
                ["read:a", "read:b", "admin:b", "admin:a", "admin:b", "admin:a"].iter()
            ),
            Ok(vec![
                make("admin", "a"),
                make("admin", "b"),
                make("read", "a"),
                make("read", "b"),
            ])
        );
        assert_eq!(
            Claim::parse_list(["read:good", "bad-stuff.*"].iter()),
            Err(Error::Syntax(
                "the given claim bad-stuff.* is not valid".to_string()
            ))
        );
    }

    #[test]
    fn test_cmp() {
        let admin_a = make("admin", "a");
        let admin_b = make("admin", "b");
        let read_a = make("read", "a");
        let read_b = make("read", "b");
        assert!(admin_a < admin_b);
        assert!(admin_a < read_a);
        assert!(admin_b < read_a);
        assert!(admin_b < read_b);
        assert_eq!(admin_b, make("admin", "b"));

        let is_eq = admin_b == make("admin", "b");
        assert!(is_eq);
    }

    #[test]
    fn test_is_exact() {
        let admin = make("admin", "");
        let admin_a = make("admin", "a");
        let read = make("read", "");
        let read_a = make("read", "a");

        assert!(admin.is_exact(&make("admin", "")));
        assert!(!admin_a.is_exact(&make("admin", "")));
        assert!(!admin.is_exact(&make("admin", "a")));
        assert!(!admin_a.is_exact(&make("admin", "")));
        assert!(!read.is_exact(&make("admin", "")));
        assert!(!read_a.is_exact(&make("admin", "")));
        assert!(!read.is_exact(&make("admin", "a")));
        assert!(!read_a.is_exact(&make("admin", "")));
    }

    #[test]
    fn test_is_exact_str() {
        let admin = make("admin", "");
        let admin_a = make("admin", "a");
        let read = make("read", "");
        let read_a = make("read", "a");

        assert!(admin.is_exact_str("admin:*"));
        assert!(!admin_a.is_exact_str("admin:*"));
        assert!(!admin.is_exact_str("admin:a"));
        assert!(!admin_a.is_exact_str("admin:*"));
        assert!(!read.is_exact_str("admin:*"));
        assert!(!read_a.is_exact_str("admin:*"));
        assert!(!read.is_exact_str("admin:a"));
        assert!(!read_a.is_exact_str("admin:*"));
    }

    #[test]
    fn test_check() {
        let admin = make("admin", "");
        let admin_a = make("admin", "a");
        let read = make("read", "");
        let read_a = make("read", "a");

        assert!(admin.check(&make("admin", "")));
        assert!(!admin_a.check(&make("admin", "")));
        assert!(admin.check(&make("admin", "a")));
        assert!(!admin_a.check(&make("admin", "")));
        assert!(!read.check(&make("admin", "")));
        assert!(!read_a.check(&make("admin", "")));
        assert!(!read.check(&make("admin", "a")));
        assert!(!read_a.check(&make("admin", "")));
    }

    #[test]
    fn test_check_str() {
        let admin = make("admin", "");
        let admin_a = make("admin", "a");
        let read = make("read", "");
        let read_a = make("read", "a");

        assert!(admin.check_str("admin:*"));
        assert!(!admin_a.check_str("admin:*"));
        assert!(admin.check_str("admin:a"));
        assert!(!admin_a.check_str("admin:*"));
        assert!(!read.check_str("admin:*"));
        assert!(!read_a.check_str("admin:*"));
        assert!(!read.check_str("admin:a"));
        assert!(!read_a.check_str("admin:*"));
    }

    #[test]
    fn test_direct_child() {
        let claim_global = make("admin", "");
        let claim = make("admin", "paco");
        let claim_nested = make("admin", "paco.something");

        assert_eq!(
            claim.direct_child(&make("admin", "")),
            Some("paco".to_string())
        );
        assert_eq!(claim.direct_child(&make("read", "")), None);
        assert_eq!(claim.direct_child(&make("admin", "paco")), None);
        assert_eq!(claim.direct_child(&make("admin", "paco.stuff")), None);
        assert_eq!(claim_global.direct_child(&make("admin", "")), None);
        assert_eq!(claim_nested.direct_child(&make("admin", "")), None);
    }

    #[test]
    fn test_direct_child_str() {
        let claim_global = make("admin", "");
        let claim = make("admin", "paco");
        let claim_nested = make("admin", "paco.something");

        assert_eq!(claim.direct_child_str("admin:*"), Some("paco".to_string()));
        assert_eq!(claim.direct_child_str("read:*"), None);
        assert_eq!(claim.direct_child_str("admin:paco"), None);
        assert_eq!(claim.direct_child_str("admin:paco.stuff"), None);
        assert_eq!(claim_global.direct_child_str("admin:*"), None);
        assert_eq!(claim_nested.direct_child_str("admin:*"), None);
    }

    #[test]
    fn test_direct_descendant() {
        let claim_global = make("admin", "");
        let claim = make("admin", "paco");
        let claim_nested = make("admin", "paco.something");

        let paco = Some("paco".to_string());

        assert_eq!(claim.direct_descendant(&make("admin", "")), paco);
        assert_eq!(claim.direct_descendant(&make("read", "")), None);
        assert_eq!(claim.direct_descendant(&make("admin", "paco")), None);
        assert_eq!(claim.direct_descendant(&make("admin", "paco.stuff")), None);
        assert_eq!(claim_global.direct_descendant(&make("admin", "")), None);
        assert_eq!(claim_nested.direct_descendant(&make("admin", "")), paco);
    }

    #[test]
    fn test_direct_descendant_str() {
        let claim_global = make("admin", "");
        let claim = make("admin", "paco");
        let claim_nested = make("admin", "paco.something");

        let paco = Some("paco".to_string());

        assert_eq!(claim.direct_descendant_str("admin:*"), paco);
        assert_eq!(claim.direct_descendant_str("read:*"), None);
        assert_eq!(claim.direct_descendant_str("admin:paco"), None);
        assert_eq!(claim.direct_descendant_str("admin:paco.stuff"), None);
        assert_eq!(claim_global.direct_descendant_str("admin:*"), None);
        assert_eq!(claim_nested.direct_descendant_str("admin:*"), paco);
    }
}
