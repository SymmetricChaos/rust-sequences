use num::{Integer, rational::Ratio};

use crate::{
    core::traits::Increment,
    transforms::complement::Complement,
    utils::divisibility::{cototient, divisors, is_prime, totient},
};

/// The totient of each positive integer. For each positive integer n, the number of positive integers less than n which are coprime to n.
///
/// 1, 1, 2, 2, 4, 2, 6, 4, 6, 4...
pub struct Totients {
    ctr: u64,
}

impl Totients {
    /// Only u64 output is supported.
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for Totients {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr = self.ctr.checked_add(1)?;
        Some(crate::utils::divisibility::totient(self.ctr))
    }
}

/// The cototient of each positive integer. Each positive integer n, minus the number of positive integers less than n which are coprime to n.
///
/// 0, 1, 1, 2, 1, 4, 1, 4, 3, 6...
pub struct Cototients {
    ctr: u64,
}

impl Cototients {
    /// Only u64 output is supported.
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for Cototients {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr = self.ctr.checked_add(1)?;
        Some(cototient(self.ctr))
    }
}

/// The number of numbers which have totient n for each positive integer n.
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

/// The nontotient numbers. Posititve integers that do not appear in the range of Euler's totient function. Includes all odd naturals except 1. Also see EvenNontotient.
///
/// 3, 5, 7, 9, 11, 13, 14, 15, 17, 19, 21...
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
/// 14, 26, 34, 38, 50, 62, 68, 74, 76, 86...
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

/// The range of Euler's totient function.
///
///
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
    Totients::new(),       [1, 1, 2, 2, 4, 2, 6, 4, 6, 4, 10, 4, 12, 6, 8, 8, 16, 6, 18, 8, 12, 10, 22, 8, 20, 12, 18, 12, 28, 8, 30, 16, 20, 16, 24, 12, 36, 18, 24, 16, 40, 12, 42, 20, 24, 22, 46, 16, 42, 20, 32, 24, 52, 18, 40, 24, 36, 28, 58, 16, 60, 30, 36, 32, 48, 20, 66, 32, 44];
    Cototients::new(),     [0, 1, 1, 2, 1, 4, 1, 4, 3, 6, 1, 8, 1, 8, 7, 8, 1, 12, 1, 12, 9, 12, 1, 16, 5, 14, 9, 16, 1, 22, 1, 16, 13, 18, 11, 24, 1, 20, 15, 24, 1, 30, 1, 24, 21, 24, 1, 32, 7, 30, 19, 28, 1, 36, 15, 32, 21, 30, 1, 44, 1, 32, 27, 32, 17, 46, 1, 36, 25, 46, 1, 48, 1, 38, 35, 40, 17, 54, 1, 48, 27];
    TotientCount::new(),   [2, 3, 0, 4, 0, 4, 0, 5, 0, 2, 0, 6, 0, 0, 0, 6, 0, 4, 0, 5, 0, 2, 0, 10, 0, 0, 0, 2, 0, 2, 0, 7, 0, 0, 0, 8, 0, 0, 0, 9, 0, 4, 0, 3, 0, 2, 0, 11, 0, 0, 0, 2, 0, 2, 0, 3, 0, 2, 0, 9, 0, 0, 0, 8, 0, 2, 0, 0, 0, 2, 0, 17, 0, 0, 0, 0, 0, 2, 0, 10, 0, 2, 0, 6, 0, 0, 0, 6, 0, 0, 0, 3];
    EvenNontotient::new(), [14, 26, 34, 38, 50, 62, 68, 74, 76, 86, 90, 94, 98, 114, 118, 122, 124, 134, 142, 146, 152, 154, 158, 170, 174, 182, 186, 188, 194, 202, 206, 214, 218, 230, 234, 236, 242, 244, 246, 248, 254, 258, 266, 274, 278, 284, 286, 290, 298, 302, 304, 308, 314, 318];
    Nontotient::new(),     [3, 5, 7, 9, 11, 13, 14, 15, 17, 19, 21, 23, 25, 26, 27, 29, 31, 33, 34, 35, 37, 38, 39, 41, 43, 45, 47, 49, 50, 51, 53, 55, 57, 59, 61, 62, 63, 65, 67, 68, 69, 71, 73, 74, 75, 76, 77, 79, 81, 83, 85, 86, 87, 89, 90, 91, 93, 94, 95, 97, 98, 99, 101, 103, 105, 107];
    TotientRange::new(),   [1, 2, 4, 6, 8, 10, 12, 16, 18, 20, 22, 24, 28, 30, 32, 36, 40, 42, 44, 46, 48, 52, 54, 56, 58, 60, 64, 66, 70, 72, 78, 80, 82, 84, 88, 92, 96, 100, 102, 104, 106, 108, 110, 112, 116, 120, 126, 128, 130, 132, 136, 138, 140, 144, 148, 150, 156, 160, 162, 164, 166, 168, 172, 176];
);
