use crate::{
    Number,
    utils::{divisibility::divisors, miller_rabin::is_prime, totient::totient},
};
#[cfg(feature = "big_int")]
use num::rational::Ratio;
#[cfg(not(feature = "big_int"))]
use num::{CheckedMul, rational::Ratio};

/// Positive natural numbers that appear as a totient more often than any smaller natural number.
///
/// ```text
/// 1, 2, 4, 8, 12, 24, 48, 72, 144, 240, 432, 480, 576, 720, 1152...
/// ```
pub struct HighlyTotient {
    ctr: Number,
    record: Number,
}

impl HighlyTotient {
    // If the big_int feature is not used this rapidly overflows an internal computation.
    pub fn new() -> Self {
        Self { ctr: 0, record: 0 }
    }
}

#[cfg(not(feature = "big_int"))]
impl Iterator for HighlyTotient {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.ctr == 0 {
                self.ctr += 2;
                return Some(1);
            }
            let m = self.ctr;
            let nmax = {
                let mut nmax = Ratio::new(m, 1);
                for d in divisors(m)
                    .iter()
                    .map(|n| *n + 1)
                    .filter(|n| is_prime(*n))
                    .map(|n| Ratio::new(n, n - 1))
                {
                    nmax = nmax.checked_mul(&d)?;
                }
                nmax
            };

            let mut n = m;
            let mut k = 0;
            while Ratio::new(n, 1) <= nmax {
                if totient(n) == m {
                    k += 1;
                }
                n += 1;
            }

            if k > self.record {
                let out = self.ctr;
                self.ctr += 2;
                self.record = k;
                return Some(out);
            } else {
                self.ctr += 2;
            }
        }
    }
}

#[cfg(feature = "big_int")]
impl Iterator for HighlyTotient {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            use num::{BigInt, One};

            if self.ctr == 0 {
                self.ctr += 2;
                return Some(1);
            }

            let m = self.ctr;
            // Without using BigRational this overflows after 13 terms
            let nmax: Ratio<BigInt> = divisors(m)
                .iter()
                .map(|n| *n + 1)
                .filter(|n| is_prime(*n))
                .map(|n| Ratio::new(n.into(), (n - 1).into()))
                .product::<Ratio<BigInt>>()
                * Ratio::new(m.into(), BigInt::one());
            let mut n = m;
            let mut k = 0;
            while Ratio::new(n.into(), BigInt::one()) <= nmax {
                if totient(n) == m {
                    k += 1;
                }
                n += 1;
            }

            if k > self.record {
                let out = self.ctr;
                self.ctr += 2;
                self.record = k;
                return Some(out);
            } else {
                self.ctr += 2;
            }
        }
    }
}

crate::check_sequences!(
    // only a few terms because this runs relatively slow
    HighlyTotient::new(), [1, 2, 4, 8, 12, 24, 48, 72, 144, 240, 432, 480, 576, 720, 1152, 1440];
);

crate::sample_sequences!(
    HighlyTotient::new();
);
