use num::{Integer, rational::Ratio};

use crate::{
    core::traits::Increment,
    utils::{divisibility::divisors, miller_rabin::is_prime, totient::totient},
};

/// The nontotient numbers. Posititve integers that do not appear in the range of Euler's totient function. Includes all odd naturals except 1. Also see EvenNontotient.
///
/// ```text
/// 3, 5, 7, 9, 11, 13, 14, 15, 17, 19, 21...
/// ```
pub struct Nontotient {
    ctr: u64,
}

impl Nontotient {
    /// Only u64 output is supported.
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for Nontotient {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.ctr.incr()?;
            if self.ctr.is_odd() && self.ctr != 1 {
                return Some(self.ctr);
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
                if k == 0 {
                    return Some(self.ctr);
                }
            }
        }
    }
}

/// The even nontotient numbers. Even positive integers that are not in the range of Euler's totient function. See also Nontotient which include all odd numbers except 1.
///
/// ```text
/// 14, 26, 34, 38, 50, 62, 68, 74, 76, 86...
/// ```
pub struct EvenNontotient {
    ctr: u64,
}

impl EvenNontotient {
    /// Only u64 output is supported.
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for EvenNontotient {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.ctr = self.ctr.checked_add(2)?;

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
            if k == 0 {
                return Some(self.ctr);
            }
        }
    }
}

crate::check_sequences!(
    EvenNontotient::new(), [14, 26, 34, 38, 50, 62, 68, 74, 76, 86, 90, 94, 98, 114, 118, 122, 124, 134, 142, 146, 152, 154, 158, 170, 174, 182, 186, 188, 194, 202, 206, 214, 218, 230, 234, 236, 242, 244, 246, 248, 254, 258, 266, 274, 278, 284, 286, 290, 298, 302, 304, 308, 314, 318];
    Nontotient::new(),     [3, 5, 7, 9, 11, 13, 14, 15, 17, 19, 21, 23, 25, 26, 27, 29, 31, 33, 34, 35, 37, 38, 39, 41, 43, 45, 47, 49, 50, 51, 53, 55, 57, 59, 61, 62, 63, 65, 67, 68, 69, 71, 73, 74, 75, 76, 77, 79, 81, 83, 85, 86, 87, 89, 90, 91, 93, 94, 95, 97, 98, 99, 101, 103, 105, 107];
);
