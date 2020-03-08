//! Prelude type definitions.

/// Defines minimum common operations of check digit algorithms.
pub trait CheckDigitAlgo {
    /// Returns true if the argument is valid according to the algorithm. The argument must be a
    /// *protected* string that consists of an original string and check digit(s).
    fn validate(&self, protected: &str) -> Result<bool, Error>;

    /// Generates a valid protected string for the argument according to the algorithm. This method
    /// returns a combined string of the original string and computed check digit(s).
    fn generate(&self, unprotected: &str) -> Result<String, Error>;

    /// Computes the check digit(s) for the argument. Unlike `generate()`, this method returns the
    /// check digit(s) only.
    fn compute(&self, unprotected: &str) -> Result<String, Error>;
}

use std::{error, fmt};

/// Represents possible errors of check digit computation.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Error {
    InvalidProtectedString(String),
    UnknownCharInString(String),
    #[doc(hidden)]
    __Nonexhaustive,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidProtectedString(ss) => write!(f, "invalid protected string '{}'", ss),
            Error::UnknownCharInString(c) => write!(f, "unknown character '{}' in string", c),
            Error::__Nonexhaustive => unreachable!(),
        }
    }
}
