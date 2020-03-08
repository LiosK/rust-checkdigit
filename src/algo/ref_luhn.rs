//! Reference implementation of Luhn algorithm. Note that this implementation is prepared only for
//! developing APIs of the library and will be removed in the future release.

use crate::{prelude::*, util};

/// Returns a new Luhn algorithm object.
pub fn ref_luhn() -> impl CheckDigitAlgo {
    RefLuhn::default()
}

/// Character set for Luhn algorithm.
const CHARSET: &str = "0123456789";

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct RefLuhn {
    lossy: bool,
    charmap: util::CharMap<u8>,
}

impl Default for RefLuhn {
    fn default() -> Self {
        Self {
            lossy: true,
            charmap: util::CharMap::from(CHARSET),
        }
    }
}

impl CheckDigitAlgo for RefLuhn {
    fn validate(&self, protected: &str) -> Result<bool, Error> {
        let (unprotected, check_chars) = util::split_protected_tail_n(protected, 1)?;
        Ok(check_chars == self.compute(unprotected)?)
    }

    fn generate(&self, unprotected: &str) -> Result<String, Error> {
        util::build_protected_apend(unprotected, &self.compute(unprotected)?)
    }

    fn compute(&self, unprotected: &str) -> Result<String, Error> {
        let ns = if self.lossy {
            self.charmap.convert_chars_lossy(unprotected)
        } else {
            self.charmap.convert_chars(unprotected)?
        };
        let sum = ns.iter().rev().enumerate().fold(0, |acc, (i, &n)| {
            (acc + if is_even(i) { luhn_double(n) } else { n }) % 10
        });
        Ok(self.charmap.convert_nums(&[(10 - sum) % 10]))
    }
}

#[inline]
fn is_even(n: usize) -> bool {
    n & 1 == 0
}

#[inline]
fn luhn_double(n: u8) -> u8 {
    n * 2 - if n < 5 { 0 } else { 9 }
}
