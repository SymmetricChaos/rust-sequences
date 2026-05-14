use crate::core::traits::Increment;
use num::{BigInt, CheckedAdd, CheckedMul, Integer};

/// The hyperfactorials numbers. Partial products of each positive natural number to the power of itself.
///
/// 1, 1, 4, 108, 27648, 86400000...
pub struct HyperFactorial<T> {
    prod: T,
    ctr: T,
}

impl<T: CheckedAdd + CheckedMul + Integer + Clone> HyperFactorial<T> {
    pub fn new() -> Self {
        Self {
            prod: T::one(),
            ctr: T::zero(),
        }
    }
}

impl HyperFactorial<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: CheckedAdd + CheckedMul + Integer + Clone> Iterator for HyperFactorial<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.prod.clone();
        self.ctr.incr()?;
        let p = self.ctr.clone();
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
    HyperFactorial::new_big(), [1_u128, 1, 4, 108, 27648, 86400000, 4031078400000];
);
