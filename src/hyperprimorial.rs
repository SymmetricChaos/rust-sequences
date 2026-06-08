use crate::{Number, core::Primes};
use num::{BigInt, One, Zero};

/// The hyperprimorial numbers. Partial products of each prime to the power of itself.
///
/// ```text
/// 1, 4, 108, 337500, 277945762500, 79301169838123235887500...
/// ```
pub struct HyperPrimorial<T> {
    prod: T,
    primes: Primes<T>,
}

impl HyperPrimorial<Number> {
    pub fn new() -> Self {
        Self {
            prod: 1,
            primes: Primes::new(),
        }
    }
}

#[cfg(feature = "big_int")]
impl HyperPrimorial<BigInt> {
    pub fn new_big() -> Self {
        Self {
            prod: BigInt::one(),
            primes: Primes::new_big(),
        }
    }
}

impl Iterator for HyperPrimorial<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.prod.clone();
        let p = self.primes.next()?;
        let mut ctr = p.clone();
        while !ctr.is_zero() {
            ctr = ctr - 1;
            self.prod = match self.prod.checked_mul(p) {
                Some(n) => n,
                None => return Some(out),
            };
        }

        Some(out)
    }
}

#[cfg(feature = "big_int")]
impl Iterator for HyperPrimorial<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.prod.clone();
        let p = self.primes.next()?;
        let mut ctr = p.clone();
        while !ctr.is_zero() {
            ctr = ctr - BigInt::one();
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

crate::sample_sequences!(
    HyperPrimorial::new_big();
);
