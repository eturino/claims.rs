use lazy_static;
use regex::Regex;

pub fn is_valid_claim(claim: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([\w_\-]+):(\*|(\w[\w_.\-]*\w)(\.\*)?)$").unwrap();
    }

    RE.is_match(claim)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_valid(claim: &str) {
        let msg = format!("claim: '{}' should work", claim);
        assert!(is_valid_claim(claim), msg)
    }

    fn check_invalid(claim: &str) {
        let msg = format!("claim: '{}' should fail", claim);
        assert!(!is_valid_claim(claim), msg)
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
}