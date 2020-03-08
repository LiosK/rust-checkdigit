use checkdigit::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct AlgoTester<T> {
    algo: T,
}

impl<T: CheckDigitAlgo> AlgoTester<T> {
    pub fn new(algo: T) -> Self {
        Self { algo }
    }

    pub fn execute(&self, valid_cases: &[(&str, &str, &str)], invalid_cases: &[&str]) {
        for (protected, unprotected, check_char) in valid_cases {
            self.test_valid(protected, unprotected, check_char);
        }
        for protected in invalid_cases {
            self.test_invalid(protected);
        }
    }

    #[inline]
    fn test_valid(&self, protected: &str, unprotected: &str, check_char: &str) {
        assert_eq!(self.algo.validate(protected), Ok(true));
        assert_eq!(self.algo.compute(unprotected), Ok(check_char.to_string()));
        assert_eq!(self.algo.generate(unprotected), Ok(protected.to_string()));
    }

    #[inline]
    fn test_invalid(&self, protected: &str) {
        assert_eq!(self.algo.validate(protected), Ok(false));
    }
}
