use crate::claim::CLAIM_REGEX;

pub fn is_valid_claim_str(claim_str: &str) -> bool {
    CLAIM_REGEX.is_match(claim_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_valid(claim: &str) {
        let msg = format!("claim: '{}' should work", claim);
        assert!(is_valid_claim_str(claim), msg)
    }

    fn check_invalid(claim: &str) {
        let msg = format!("claim: '{}' should fail", claim);
        assert!(!is_valid_claim_str(claim), msg)
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
