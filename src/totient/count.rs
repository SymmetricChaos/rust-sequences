use crate::{
    core::traits::Increment,
    utils::{divisibility::divisors, miller_rabin::is_prime, totient::totient},
};
use num::{Integer, rational::Ratio};

/// The count of natural number which have totient n for each positive integer n.
///
/// 2, 3, 0, 4, 0, 4, 0, 5, 0, 2, 0, 6, 0...
pub struct TotientCount {
    ctr: u64,
}

impl TotientCount {
    /// Only u64 output is supported.
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for TotientCount {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.incr()?;
        if self.ctr == 1 {
            return Some(2);
        } else if self.ctr.is_odd() {
            return Some(0);
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
            Some(k)
        }
    }
}

crate::check_sequences!(
    TotientCount::new(),   [2, 3, 0, 4, 0, 4, 0, 5, 0, 2, 0, 6, 0, 0, 0, 6, 0, 4, 0, 5, 0, 2, 0, 10, 0, 0, 0, 2, 0, 2, 0, 7, 0, 0, 0, 8, 0, 0, 0, 9, 0, 4, 0, 3, 0, 2, 0, 11, 0, 0, 0, 2, 0, 2, 0, 3, 0, 2, 0, 9, 0, 0, 0, 8, 0, 2, 0, 0, 0, 2, 0, 17, 0, 0, 0, 0, 0, 2, 0, 10, 0, 2, 0, 6, 0, 0, 0, 6, 0, 0, 0, 3];
);
