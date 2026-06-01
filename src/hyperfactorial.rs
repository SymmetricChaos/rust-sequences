use crate::{Number, core::traits::Increment};
use num::{BigInt, One, Zero};

/// The hyperfactorials numbers. Partial products of each positive natural number to the power of itself.
///
/// ```text
/// 1, 1, 4, 108, 27648, 86400000...
/// ```
pub struct HyperFactorial<T> {
    prod: T,
    ctr: T,
}

impl HyperFactorial<Number> {
    pub fn new() -> Self {
        Self { prod: 1, ctr: 0 }
    }
}

impl HyperFactorial<BigInt> {
    pub fn new_big() -> Self {
        Self {
            prod: BigInt::one(),
            ctr: BigInt::zero(),
        }
    }
}

impl Iterator for HyperFactorial<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.prod.clone();
        self.ctr.incr()?;
        let p = self.ctr;
        let mut ctr = p;
        while ctr != 0 {
            ctr = ctr - 1;
            self.prod = match self.prod.checked_mul(p) {
                Some(n) => n,
                None => return Some(out),
            };
        }

        Some(out)
    }
}

impl Iterator for HyperFactorial<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.prod.clone();
        self.ctr.incr()?;
        let p = self.ctr.clone();
        let mut ctr = p.clone();
        while !ctr.is_zero() {
            ctr = ctr - 1;
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
