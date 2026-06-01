use crate::{Number, prime_gaps::PrimeGaps};
use num::BigInt;

/// The Mersenne numbers. 2^p-1 for all primes p.
///
/// ```text
/// 3, 7, 31, 127, 2047, 8191, 131071...
/// ```
pub struct Mersenne<T> {
    gaps: PrimeGaps<usize>,
    ctr: T,
    overflowed: bool,
}

impl Mersenne<Number> {
    pub fn new() -> Self {
        Self {
            gaps: PrimeGaps::new(),
            ctr: 4,
            overflowed: false,
        }
    }
}

impl Mersenne<BigInt> {
    pub fn new_big() -> Self {
        Self {
            gaps: PrimeGaps::new(),
            ctr: BigInt::from(4),
            overflowed: false,
        }
    }
}

impl Iterator for Mersenne<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        if self.overflowed {
            return None;
        }

        let p = self.gaps.next()?;
        let out = self.ctr - 1;

        for _ in 0..p {
            match self.ctr.checked_mul(2) {
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

impl Iterator for Mersenne<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let p = self.gaps.next()?;

        let out = self.ctr.clone() - 1;

        for _ in 0..p {
            self.ctr *= 2;
        }

        Some(out)
    }
}

crate::check_sequences!(
    Mersenne::new_big(), [3_u64, 7, 31, 127, 2047, 8191, 131071, 524287, 8388607, 536870911, 2147483647, 137438953471, 2199023255551, 8796093022207, 140737488355327, 9007199254740991, 576460752303423487, 2305843009213693951];
);
