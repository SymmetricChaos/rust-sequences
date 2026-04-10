use num::{BigInt, CheckedAdd, CheckedMul, Integer};
use std::hash::Hash;

use crate::prime_gaps::PrimeGaps;

/// The Mersenne numbers. 2^p-1 for all primes p.
///
/// 3, 7, 31, 127, 2047, 8191, 131071...
pub struct Mersenne<T> {
    gaps: PrimeGaps<usize>,
    ctr: T,
    overflowed: bool,
}

impl<T: CheckedAdd + Clone + Integer + Hash> Mersenne<T> {
    pub fn new() -> Self {
        Self {
            gaps: PrimeGaps::new(),
            ctr: T::one() + T::one() + T::one() + T::one(),
            overflowed: false,
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
        if self.overflowed {
            return None;
        }

        let p = self.gaps.next()?;
        let two = T::one() + T::one();

        let out = self.ctr.clone() - T::one();

        for _ in 0..p {
            match self.ctr.checked_mul(&two) {
                Some(n) => self.ctr = n,
                None => {
                    self.overflowed = true;
                    return Some(out);
                }
            };
        }

        Some(out)
    }
}

crate::check_sequences!(
    Mersenne::<i32>::new(), [3, 7, 31, 127, 2047, 8191, 131071, 524287, 8388607];
);
