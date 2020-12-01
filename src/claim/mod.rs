mod claim_check;
mod claim_from_str;
mod is_valid_claim;
use lazy_static::lazy_static; // 1.3.0
use regex::Regex;

lazy_static! {
    static ref CLAIM_REGEX: Regex =
        Regex::new(r"^([\w_\-]+):(\*|(\w[\w_.\-]*\w)(\.\*)?)$").unwrap();
}

/// tuple for (verb, subject)
pub type Claim<'a> = (&'a str, &'a str);
