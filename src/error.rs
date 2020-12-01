use std::fmt;
use std::iter::repeat;

// based on the Error from regex

/// An error that occurred during parsing or compiling a regular expression.
#[derive(Clone, PartialEq, Debug)]
pub enum Error {
    /// A syntax error.
    Syntax(String),

    /// Hints that destructuring should not be exhaustive.
    ///
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    __NonExhaustive,
}

impl ::std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Syntax(ref err) => err.fmt(f),
            Error::__NonExhaustive => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Display;

    #[test]
    fn test_fmt() {
        let e = Error::Syntax("something".to_string());
        assert_eq!(format!("{}", e), "something");
    }
}
