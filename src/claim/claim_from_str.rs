use crate::claim::{Claim, CLAIM_REGEX};
use crate::error::Error;

pub fn claim_from_str(claim_str: &str) -> Result<Claim, Error> {
    let c = CLAIM_REGEX.captures(claim_str);

    if c.is_none() {
        return Err(err_not_parsed(claim_str));
    }

    let caps = c.unwrap();

    let verb = caps.get(1).unwrap().as_str();
    let subject_match = caps.get(2).unwrap().as_str();

    let subject = parse_subject(subject_match);
    Ok((verb, subject))
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
