use crate::core::Primes;
use num::{BigInt, CheckedAdd, Integer};
use std::hash::Hash;

/// The Copeland-Erdős constants. Infinite words formed by listing the digits of the prime natural numbers in a given base.
///
/// ```text
/// for base 10
/// 2, 3, 5, 7, 1, 1, 1, 3, 1, 7, 1, 9, 2, 3, 2, 9...
/// for base 2
/// 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1...
/// ```
pub struct CopelandErdos<T> {
    primes: Primes<T>,
    digits: Vec<T>,
    base: T,
}

impl<T: Integer + CheckedAdd + Clone + Hash> CopelandErdos<T> {
    pub fn new(base: T) -> Self {
        Self {
            primes: Primes::new(),
            digits: Vec::new(),
            base,
        }
    }
}

impl CopelandErdos<BigInt> {
    pub fn new_big<G>(base: G) -> Self
    where
        BigInt: From<G>,
    {
        Self::new(BigInt::from(base))
    }
}

impl<T: Integer + CheckedAdd + Clone + Hash> Iterator for CopelandErdos<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.digits.is_empty() {
            let mut n = self.primes.next()?;
            while n > T::zero() {
                let (div, rem) = n.div_rem(&self.base);
                self.digits.push(rem);
                n = div;
            }
        }
        self.digits.pop()
    }
}

crate::check_sequences!(
    CopelandErdos::new_big(2),  [1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1];
    CopelandErdos::new_big(10), [2, 3, 5, 7, 1, 1, 1, 3, 1, 7, 1, 9, 2, 3, 2, 9, 3, 1, 3, 7, 4, 1, 4, 3, 4, 7, 5, 3, 5, 9, 6, 1, 6, 7, 7, 1, 7, 3, 7, 9, 8, 3, 8, 9, 9, 7, 1, 0, 1, 1, 0, 3, 1, 0, 7, 1, 0, 9, 1, 1, 3, 1, 2, 7, 1, 3, 1, 1, 3, 7, 1, 3, 9, 1, 4, 9, 1, 5, 1, 1, 5, 7, 1, 6, 3, 1, 6, 7, 1, 7, 3, 1, 7, 9, 1, 8, 1, 1, 9, 1, 1];
);
