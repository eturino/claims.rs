use crate::claim::{Claim, CLAIM_REGEX};
use crate::error::Error;

pub fn claim_from_str(claim_str: &str) -> Result<Claim, Error> {
    let c = CLAIM_REGEX.captures(claim_str);

    if c.is_none() {
        return Err(err_not_parsed(claim_str));
    }

    let caps = c.unwrap();

    let verb = String::from(caps.get(1).unwrap().as_str());
    let subject_match = caps.get(2).unwrap().as_str();

    let subject = parse_subject(subject_match);
    Ok(Claim::new(&verb, &subject))
}

pub fn claims_from_strs<'a, I>(claim_strs: I) -> Result<Vec<Claim>, Error>
where
    I: Iterator<Item = &'a &'a str>,
{
    let parsed = claim_strs.map(|c| claim_from_str(c));
    let list: Result<Vec<Claim>, Error> = parsed.collect();
    match list {
        Ok(_) => {
            let mut vec = list.unwrap();
            vec.sort();
            vec.dedup();
            Ok(vec)
        }
        _ => list,
    }
}

fn parse_subject(s: &str) -> String {
    match s {
        "*" | "" => String::from(""),
        _ if s.ends_with(".*") => {
            let len = s.len();
            String::from(&s[..len - 2])
        }
        _ if s.ends_with(".") => {
            let len = s.len();
            String::from(&s[..len - 1])
        }
        _ => String::from(s),
    }
}

fn err_not_parsed(claim_str: &str) -> Error {
    Error::Syntax(format!("the given claim {} is not valid", claim_str))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_valid(claim: &str) {
        let res = claim_from_str(claim);
        let msg = format!("claim: '{}' should work but failed with {:?}", claim, res);
        assert!(res.is_ok(), msg)
    }

    fn check_invalid(claim: &str) {
        let res = claim_from_str(claim);
        let msg = format!("claim: '{}' should fail", claim);
        assert!(res.is_err(), msg)
    }

    #[test]
    fn accepts_the_valid() {
        let list = [
            "admin:some-like_this.stuff-or_o_.even-with-99",
            "read:some-like_this.stuff-or_o.even-with-99",
            "admin:something",
            "read:something",
            "A:1.9",
            "A:1-9",
            "A:*",
            "A:some.stuff.*",
        ];
        for x in list.iter() {
            check_valid(x)
        }
    }

    #[test]
    fn rejects_the_invalid() {
        let list = [
            "noverb",
            "admin:stuff-has-spaces ",
            "  admin:stuff-has-spaces",
            "  admin:stuff-has-spaces ",
            "admin:stuff:has-other-colons",
            "read:**",
            "read:.",
            "read:.paco",
            "read:*.*",
            "read:*.some.stuff",
        ];
        for x in list.iter() {
            check_invalid(x);
        }
    }

    #[test]
    fn parse_subject_for_global() {
        assert_eq!(parse_subject(""), "");
        assert_eq!(parse_subject("*"), "");
    }

    #[test]
    fn parse_subject_for_suffix() {
        assert_eq!(parse_subject("a"), "a");
        assert_eq!(parse_subject("a."), "a");
        assert_eq!(parse_subject("a.*"), "a");
        assert_eq!(parse_subject("paco"), "paco");
        assert_eq!(parse_subject("paco.el.flaco"), "paco.el.flaco");
        assert_eq!(parse_subject("paco.el.flaco."), "paco.el.flaco");
        assert_eq!(parse_subject("paco.el.flaco.*"), "paco.el.flaco");
    }

    #[test]
    fn parse_list_all_good() {
        let strings = ["read:something", "read:*", "read:*", "read:something"];
        let expected = vec![Claim::new("read", ""), Claim::new("read", "something")];

        assert_eq!(claims_from_strs(strings.iter()), Ok(expected));
    }

    #[test]
    fn parse_list_some_bad() {
        let strings = ["read:*", "read:something", "bad", "another-bad"];

        assert_eq!(claims_from_strs(strings.iter()), Err(err_not_parsed("bad")));
    }

    #[test]
    fn parse_list_blank() {
        let strings: Vec<&str> = Vec::new();
        let expected: Vec<Claim> = Vec::new();
        assert_eq!(claims_from_strs(strings.iter()), Ok(expected));
    }
}
