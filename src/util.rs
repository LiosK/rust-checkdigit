//! Crate-level common functions.

use crate::Error;

mod charmap;
pub use charmap::CharMap;

/// Splits a string at `n`th last character position, returning a `(head, tail)` tuple where `tail`
/// consists of the last `n` characters of `protected` and `head` represents the remainings.
#[inline]
pub fn split_protected_tail_n(protected: &str, n: usize) -> Result<(&str, &str), Error> {
    if let Some((mid, _)) = protected.char_indices().nth_back(n - 1) {
        Ok(protected.split_at(mid))
    } else {
        Err(Error::InvalidProtectedString(String::from(protected)))
    }
}

/// Creates a new `String` by simply apending `check_chars` to `unprotected`.
#[inline]
pub fn build_protected_apend(unprotected: &str, check_chars: &str) -> Result<String, Error> {
    let mut protected = String::with_capacity(unprotected.len() + check_chars.len());
    protected.push_str(unprotected);
    protected.push_str(check_chars);
    Ok(protected)
}

#[cfg(test)]
mod tests {
    #[test]
    fn split_protected_tail_n() {
        let (head, tail) = super::split_protected_tail_n("helloworld", 5).unwrap();
        assert_eq!(head, "hello");
        assert_eq!(tail, "world");
    }

    #[test]
    fn build_protected_apend() {
        let ss = super::build_protected_apend("hello", "world");
        assert_eq!(ss, Ok(String::from("helloworld")));
    }
}
