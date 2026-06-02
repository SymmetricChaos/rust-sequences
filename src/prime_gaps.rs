use crate::{Number, core::primes::Primes};
use num::{BigInt, CheckedAdd, Integer};
use std::hash::Hash;

/// The gaps between the primes.
///
/// ```text
/// 1, 2, 2, 4, 2, 4, 2, 4, 6, 2, 6, 4...
/// ```
pub struct PrimeGaps<T> {
    primes: Primes<T>,
    prev: T,
}

impl PrimeGaps<Number> {
    pub fn new() -> Self {
        let mut primes = Primes::new();
        primes.next();
        Self { primes, prev: 2 }
    }
}

impl PrimeGaps<BigInt> {
    pub fn new_big() -> Self {
        let mut primes = Primes::new_big();
        primes.next();
        Self {
            primes,
            prev: BigInt::from(2),
        }
    }
}

impl<T: CheckedAdd + Hash + Integer + Clone> Iterator for PrimeGaps<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let p = self.primes.next()?;
        let dif = p.clone() - self.prev.clone();
        self.prev = p.clone();
        Some(dif)
    }
}

crate::check_sequences!(
    PrimeGaps::new(), [1, 2, 2, 4, 2, 4, 2, 4, 6, 2, 6, 4, 2, 4, 6, 6, 2, 6, 4, 2, 6, 4, 6, 8, 4, 2, 4, 2, 4, 14, 4, 6, 2, 10, 2, 6, 6, 4, 6, 6, 2, 10, 2, 4, 2, 12, 12, 4, 2, 4, 6, 2, 10, 6, 6, 6, 2, 6, 4, 2, 10, 14, 4, 2, 4, 14, 6, 10, 2, 4, 6, 8, 6, 6, 4, 6, 8, 4, 8, 10, 2, 10, 2, 6, 4, 6, 8, 4, 2, 4, 12, 8, 4, 8, 4, 6, 12];
);
