use crate::{
    core::traits::Increment,
    utils::{divisibility::divisors, miller_rabin::is_prime, totient::totient},
};
use num::{Integer, rational::Ratio};

/// The range of Euler's totient function in increasing order.
///
/// 1, 2, 4, 6, 8, 10, 12, 16, 18...
pub struct TotientRange {
    ctr: u64,
}

impl TotientRange {
    /// Only u64 output is supported.
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for TotientRange {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.ctr.incr()?;
            if self.ctr == 1 {
                return Some(1);
            }
            if self.ctr.is_odd() {
                // skip it
            } else {
                let m = self.ctr;
                let nmax: Ratio<u64> = divisors(m)
                    .iter()
                    .map(|n| *n + 1)
                    .filter(|n| is_prime(*n))
                    .map(|n| Ratio::new(n, n - 1))
                    .product::<Ratio<u64>>()
                    * m;
                let mut n = m;
                let mut k = 0;
                while Ratio::new(n, 1) <= nmax {
                    if totient(n) == m {
                        k += 1;
                    }
                    n += 1;
                }
                if k != 0 {
                    return Some(self.ctr);
                }
            }
        }
    }
}

crate::check_sequences!(
    TotientRange::new(),   [1, 2, 4, 6, 8, 10, 12, 16, 18, 20, 22, 24, 28, 30, 32, 36, 40, 42, 44, 46, 48, 52, 54, 56, 58, 60, 64, 66, 70, 72, 78, 80, 82, 84, 88, 92, 96, 100, 102, 104, 106, 108, 110, 112, 116, 120, 126, 128, 130, 132, 136, 138, 140, 144, 148, 150, 156, 160, 162, 164, 166, 168, 172, 176];
);
