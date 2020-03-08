use crate::Error;
use fnv::FnvHashMap;
use std::hash::Hash;

/// Provides character-to-numerical-value and numerical-value-to-character converters.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct CharMap<T: Hash + Eq> {
    c_to_n: FnvHashMap<char, T>,
    n_to_c: FnvHashMap<T, char>,
}

impl<T: Hash + Eq + Clone> CharMap<T> {
    /// Creates char-to-num and num-to-char maps from a charset string and numerical value set.
    ///
    /// # Panics
    ///
    /// Panics if the lengths of both sets do not match or if either set has a duplicate element.
    pub fn with_numset(charset: &str, numset: &[T]) -> Self {
        let instance = Self {
            c_to_n: charset.chars().zip(numset.iter().cloned()).collect(),
            n_to_c: numset.iter().cloned().zip(charset.chars()).collect(),
        };

        let charset_size = charset.chars().count();
        assert_eq!(
            numset.len(),
            charset_size,
            "`charset` and `numset` must have the same length"
        );
        assert_eq!(
            instance.c_to_n.len(),
            charset_size,
            "every character in `charset` string must be unique"
        );
        assert_eq!(
            instance.n_to_c.len(),
            charset_size,
            "every element in `numset` list must be unique"
        );

        instance
    }

    /// Converts a string into a sequence of corresponding numerical values. Returns `Err` if the
    /// string includes a unknown character.
    pub fn convert_chars(&self, cs: &str) -> Result<Vec<T>, Error> {
        cs.chars()
            .map(|c| {
                self.c_to_n
                    .get(&c)
                    .cloned()
                    .ok_or_else(|| Error::UnknownCharInString(c.to_string()))
            })
            .collect()
    }

    /// Converts a string into a sequence of corresponding numerical values. This methond provides
    /// lossy conversion; unknown characters in the string are just ignored.
    pub fn convert_chars_lossy(&self, cs: &str) -> Vec<T> {
        cs.chars()
            .filter_map(|c| self.c_to_n.get(&c))
            .cloned()
            .collect()
    }

    /// Converts a sequence of numerical values into corresponding characters and returns a
    /// concatenated string.
    ///
    /// # Panics
    ///
    /// Panics if the sequence incldues a unknown value.
    pub fn convert_nums(&self, ns: &[T]) -> String {
        ns.iter().map(|n| self.n_to_c[n]).collect()
    }
}

impl From<&str> for CharMap<u8> {
    /// Takes a sequence of unique characters and creates maps between the characters and unsigned
    /// integers starting from zero (0, 1, 2, ...).
    fn from(charset: &str) -> Self {
        let numset: Vec<_> = (0..).take(charset.chars().count()).collect();
        Self::with_numset(charset, &numset)
    }
}

impl From<&str> for CharMap<u16> {
    /// Takes a sequence of unique characters and creates maps between the characters and unsigned
    /// integers starting from zero (0, 1, 2, ...).
    fn from(charset: &str) -> Self {
        let numset: Vec<_> = (0..).take(charset.chars().count()).collect();
        Self::with_numset(charset, &numset)
    }
}

impl From<&str> for CharMap<u32> {
    /// Takes a sequence of unique characters and creates maps between the characters and unsigned
    /// integers starting from zero (0, 1, 2, ...).
    fn from(charset: &str) -> Self {
        let numset: Vec<_> = (0..).take(charset.chars().count()).collect();
        Self::with_numset(charset, &numset)
    }
}

#[cfg(test)]
mod tests {
    use super::CharMap;
    use crate::Error;

    const ALPHABETIC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    #[test]
    fn from_str() {
        assert_eq!(
            CharMap::<u8>::from("ABCD"),
            CharMap::with_numset("ABCD", &vec![0, 1, 2, 3])
        );
        assert_eq!(
            CharMap::<u16>::from("ABCD"),
            CharMap::with_numset("ABCD", &vec![0, 1, 2, 3])
        );
        assert_eq!(
            CharMap::<u32>::from("ABCD"),
            CharMap::with_numset("ABCD", &vec![0, 1, 2, 3])
        );
    }

    #[test]
    #[should_panic]
    fn from_str_duplicate() {
        CharMap::<u8>::from("0123405");
    }

    #[test]
    fn with_numset() {
        assert_eq!(
            CharMap::with_numset("0123", &[5, 6, 7, 8]),
            CharMap::with_numset("0123", &vec![5, 6, 7, 8]),
        );
    }

    #[test]
    #[should_panic]
    fn with_numset_different_lengths() {
        CharMap::with_numset("0123", &[5, 6, 7]);
    }

    #[test]
    #[should_panic]
    fn with_numset_duplicate_char() {
        CharMap::with_numset("0113", &[5, 6, 7, 8]);
    }

    #[test]
    #[should_panic]
    fn with_numset_duplicate_num() {
        CharMap::with_numset("0123", &[5, 6, 7, 7]);
    }

    #[test]
    fn convert_chars() {
        let cm: CharMap<u8> = CharMap::from(ALPHABETIC);
        assert_eq!(
            cm.convert_chars("ACBDADBC"),
            Ok(vec![0, 2, 1, 3, 0, 3, 1, 2])
        );
        assert_eq!(
            cm.convert_chars("AB+CD-CB"),
            Err(Error::UnknownCharInString(String::from("+")))
        );
    }

    #[test]
    fn convert_chars_lossy() {
        let cm: CharMap<u8> = CharMap::from(ALPHABETIC);
        assert_eq!(
            cm.convert_chars_lossy("ACBDADBC"),
            vec![0, 2, 1, 3, 0, 3, 1, 2]
        );
        assert_eq!(cm.convert_chars_lossy("AB+CD-CB"), vec![0, 1, 2, 3, 2, 1]);
    }

    #[test]
    fn convert_nums() {
        let cm: CharMap<u8> = CharMap::from(ALPHABETIC);
        assert_eq!(cm.convert_nums(&[0, 2, 1, 3, 0, 3, 1, 2]), "ACBDADBC");
    }

    #[test]
    #[should_panic]
    fn convert_nums_unknown() {
        let cm: CharMap<u8> = CharMap::from(ALPHABETIC);
        cm.convert_nums(&[0, 2, 1, 3, 0, 3, 64, 1, 2]);
    }
}
