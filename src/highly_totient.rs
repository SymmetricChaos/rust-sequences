use crate::utils::divisibility::{divisors, is_prime, totient};
use num::BigRational;

/// Positive natural numbers that appear as a totient more often than any smaller natural number.
///
/// 1, 2, 4, 8, 12, 24, 48, 72, 144...
pub struct HighlyTotient {
    ctr: u64,
    record: u64,
}

impl HighlyTotient {
    /// Only u64 output is supported.
    pub fn new() -> Self {
        Self { ctr: 0, record: 0 }
    }
}

impl Iterator for HighlyTotient {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.ctr == 0 {
                self.ctr += 2;
                return Some(1);
            }

            let m = self.ctr;
            // Without using BigRational this overflows after 13 terms
            let nmax: BigRational = divisors(m)
                .iter()
                .map(|n| *n + 1)
                .filter(|n| is_prime(*n))
                .map(|n| BigRational::new(n.into(), (n - 1).into()))
                .product::<BigRational>()
                * BigRational::new(m.into(), 1.into());
            let mut n = m;
            let mut k = 0;
            while BigRational::new(n.into(), 1.into()) <= nmax {
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
