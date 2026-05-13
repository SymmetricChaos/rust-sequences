use crate::core::Primes;
use num::{BigInt, CheckedAdd, CheckedMul, Integer};
use std::hash::Hash;

/// The hyperprimorial numbers. Partial products of each prime to the power of itself.
///
/// 4, 108, 337500, 277945762500...
pub struct HyperPrimorial<T> {
    prod: T,
    primes: Primes<T>,
}

impl<T: CheckedAdd + CheckedMul + Hash + Integer + Clone> HyperPrimorial<T> {
    pub fn new() -> Self {
        Self {
            prod: T::one(),
            primes: Primes::new(),
        }
    }
}

impl HyperPrimorial<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: CheckedAdd + CheckedMul + Hash + Integer + Clone> Iterator for HyperPrimorial<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.prod.clone();
        let p = self.primes.next()?;
        let mut ctr = p.clone();
        while !ctr.is_zero() {
            ctr = ctr - T::one();
            self.prod = match self.prod.checked_mul(&p) {
                Some(n) => n,
                None => return Some(out),
            };
        }

        Some(out)
    }
}

crate::check_sequences!(
    HyperPrimorial::new_big(), [1_u128, 4, 108, 337500, 277945762500, 79301169838123235887500];
);
