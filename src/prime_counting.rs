use crate::{Number, core::primes::Primes};
use num::{BigInt, CheckedAdd, Integer, One, Zero};
use std::hash::Hash;

/// The prime counting function evaluated at each positive integer.
///
/// ```text
/// 0, 1, 2, 2, 3, 3, 4, 4, 4, 4, 5, 5, 6, 6, 6, 6, 7, 7, 8, 8, 8, 8, 9...
/// ```
pub struct PrimeCounting<T> {
    prime: Primes<T>,
    next_prime: T,
    n: T,
    ctr: T,
}

impl PrimeCounting<Number> {
    pub fn new() -> Self {
        let mut prime = Primes::new();
        let next_prime = prime.next().unwrap();
        Self {
            prime,
            next_prime,
            n: 1,
            ctr: 0,
        }
    }
}

#[cfg(feature = "big_int")]
impl PrimeCounting<BigInt> {
    pub fn new_big() -> Self {
        let mut prime = Primes::new();
        let next_prime = prime.next().unwrap();
        Self {
            prime,
            next_prime,
            n: BigInt::one(),
            ctr: BigInt::zero(),
        }
    }
}

impl<T: CheckedAdd + Clone + Hash + Integer> Iterator for PrimeCounting<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.ctr.clone();

        self.n = self.n.checked_add(&T::one())?;
        if self.n == self.next_prime {
            self.ctr = self.ctr.checked_add(&T::one())?;
            self.next_prime = self.prime.next()?;
        }

        Some(out)
    }
}

crate::check_sequences!(
    PrimeCounting::new(), [0, 1, 2, 2, 3, 3, 4, 4, 4, 4, 5, 5, 6, 6, 6, 6, 7, 7, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 10, 10, 11, 11, 11, 11, 11, 11, 12, 12, 12, 12, 13, 13, 14, 14, 14, 14, 15, 15, 15, 15, 15, 15, 16, 16, 16, 16, 16, 16, 17, 17, 18, 18, 18, 18, 18, 18, 19, 19, 19, 19, 20, 20, 21, 21, 21, 21, 21, 21];
);

crate::sample_sequences!(
    PrimeCounting::new();
);
