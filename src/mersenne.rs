use crate::core::prime::PrimeGaps;
use num::{BigInt, CheckedAdd, CheckedMul, Integer};
use std::{fmt::Display, hash::Hash};

/// The Mersenne numbers. 2^p-1 for all primes p.
pub struct Mersenne<T> {
    gaps: PrimeGaps<T>,
    ctr: T,
}

impl<T: CheckedAdd + Clone + Integer + Hash> Mersenne<T> {
    pub fn new() -> Self {
        Self {
            gaps: PrimeGaps::new(),
            ctr: T::one() + T::one() + T::one() + T::one(),
        }
    }
}

impl Mersenne<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: CheckedAdd + CheckedMul + Clone + Integer + Hash> Iterator for Mersenne<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut p = self.gaps.next()?;
        let two = T::one() + T::one();

        let out = self.ctr.clone() - T::one();

        while !p.is_zero() {
            self.ctr = self.ctr.checked_mul(&two)?;
            p = p - T::one();
        }

        Some(out)
    }
}

crate::check_sequences!(
    Mersenne::<i32>::new(), [3, 7, 31, 127, 2047, 8191, 131071, 524287, 8388607];
);
