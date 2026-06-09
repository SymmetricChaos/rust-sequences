use crate::{
    Number,
    core::traits::Increment,
    utils::{divisibility::divisors, miller_rabin::is_prime, totient::totient},
};
use num::{Integer, rational::Ratio};

/// The count of natural number which have totient n for each positive integer n.
///
/// ```text
/// 2, 3, 0, 4, 0, 4, 0, 5, 0, 2, 0, 6, 0, 0, 0, 6, 0, 4, 0, 5, 0, 2, 0...
/// ```
pub struct TotientCount {
    ctr: Number,
}

impl TotientCount {
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
            let nmax: Ratio<Number> = divisors(m)
                .iter()
                .map(|n| *n + 1)
                .filter(|n| is_prime(*n))
                .map(|n| Ratio::new(n, n - 1))
                .product::<Ratio<Number>>()
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
    TotientCount::new(), [2, 3, 0, 4, 0, 4, 0, 5, 0, 2, 0, 6, 0, 0, 0, 6, 0, 4, 0, 5, 0, 2, 0, 10, 0, 0, 0, 2, 0, 2, 0, 7, 0, 0, 0, 8, 0, 0, 0, 9, 0, 4, 0, 3, 0, 2, 0, 11, 0, 0, 0, 2, 0, 2, 0, 3, 0, 2, 0, 9, 0, 0, 0, 8, 0, 2, 0, 0, 0, 2, 0, 17, 0, 0, 0, 0, 0, 2, 0, 10, 0, 2, 0, 6, 0, 0, 0, 6, 0, 0, 0, 3];
);

crate::sample_sequences!(
    TotientCount::new();
);
