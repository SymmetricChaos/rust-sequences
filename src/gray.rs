use crate::{Number, core::traits::Increment};
use num::{BigInt, Zero};

/// The Gray codes, bit sequence where each term differs from the previous by a single bit. A permutation of the non-negative integers.
///
/// ```text
/// 0, 1, 3, 2, 6, 7, 5, 4, 12, 13, 15, 14, 10, 11, 9, 8, 24, 25, 27...
/// ```
pub struct Gray<T> {
    ctr: T,
}

impl Gray<Number> {
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

#[cfg(feature = "big_int")]
impl Gray<BigInt> {
    pub fn new_big() -> Self {
        Self {
            ctr: BigInt::zero(),
        }
    }
}

impl Iterator for Gray<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.ctr.clone() ^ (self.ctr.clone() >> 1_usize);
        self.ctr.incr()?;
        Some(out)
    }
}

#[cfg(feature = "big_int")]
impl Iterator for Gray<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.ctr.clone() ^ (self.ctr.clone() >> 1_usize);
        self.ctr.incr()?;
        Some(out)
    }
}

crate::check_sequences!(
    Gray::new_big(), [0, 1, 3, 2, 6, 7, 5, 4, 12, 13, 15, 14, 10, 11, 9, 8, 24, 25, 27, 26, 30, 31, 29, 28, 20, 21, 23, 22, 18, 19, 17, 16, 48, 49, 51, 50, 54, 55, 53, 52, 60, 61, 63, 62, 58, 59, 57, 56, 40, 41, 43, 42, 46, 47, 45, 44, 36, 37, 39, 38, 34, 35, 33, 32, 96, 97, 99, 98, 102, 103, 101];
    Gray::new(),     [0, 1, 3, 2, 6, 7, 5, 4, 12, 13, 15, 14, 10, 11, 9, 8, 24, 25, 27, 26, 30, 31, 29, 28, 20, 21, 23, 22, 18, 19, 17, 16, 48, 49, 51, 50, 54, 55, 53, 52, 60, 61, 63, 62, 58, 59, 57, 56, 40, 41, 43, 42, 46, 47, 45, 44, 36, 37, 39, 38, 34, 35, 33, 32, 96, 97, 99, 98, 102, 103, 101];
);

crate::sample_sequences!(
    Gray::new();
);
