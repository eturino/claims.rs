mod check;
mod claim_from_str;
mod descendants;
mod is_valid_claim;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref CLAIM_REGEX: Regex =
        Regex::new(r"^([\w_\-]+):(\*|(\w[\w_.\-]*\w)(\.\*)?)$").unwrap();
}

/// tuple for (verb, subject)
pub type Claim<'a> = (&'a str, &'a str);

pub fn claim_is_global(claim: Claim) -> bool {
    claim.1.len() == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claim_is_global() {
        assert!(claim_is_global(("read", "")));
        assert!(!claim_is_global(("read", "paco")));
    }
}
