use crate::claim::{Claim, CLAIM_REGEX};
use crate::error::Error;

pub fn claim_from_str(claim_str: &str) -> Result<Claim, Error> {
    let c = CLAIM_REGEX.captures(claim_str);

    if c.is_none() {
        return Err(err_not_parsed(claim_str));
    }

    let caps = c.unwrap();

    let o_verb = caps.get(1);
    let o_subject = caps.get(2);

    if let Some(verb) = o_verb {
        let subject = o_subject.map_or("", |x| parse_subject(x.as_str()));
        Ok((verb.as_str(), subject))
    } else {
        Err(err_no_verb(claim_str))
    }
}

fn parse_subject(s: &str) -> &str {
    match s {
        "*" | "" => "",
        _ if s.ends_with(".*") => {
            let len = s.len();
            &s[..len - 2]
        }
        _ => s,
    }
}

fn err_not_parsed(claim_str: &str) -> Error {
    Error::Syntax(format!("the given claim {} is not valid", claim_str))
}

fn err_no_verb(claim_str: &str) -> Error {
    let msg = format!("the given claim {} is not valid (no verb)", claim_str);
    Error::Syntax(msg)
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
            "admin:stuff-has-spaces ",
            "  admin:stuff-has-spaces",
            "  admin:stuff-has-spaces ",
            "admin:stuff:has-other-colons",
            "read:**",
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
        assert_eq!(parse_subject("paco"), "paco");
        assert_eq!(parse_subject("paco.el.flaco"), "paco.el.flaco");
        assert_eq!(parse_subject("paco.el.flaco.*"), "paco.el.flaco");
    }
}
