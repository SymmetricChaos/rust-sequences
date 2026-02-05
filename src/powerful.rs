use crate::utils::divisibility::prime_factorization;
use num::integer::gcd;

/// Positive natural numbers that are divisible by all the squares of their prime factors.
/// 1, 4, 8, 9, 16, 25, 27, 32, 36, 49...
pub struct Powerful {
    ctr: u64,
}

impl Powerful {
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for Powerful {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.ctr = self.ctr.checked_add(1)?;
            let n = self.ctr;
            let primes = prime_factorization(self.ctr);
            if primes.iter().map(|p| p.1).all(|p| p >= 2) {
                return Some(n);
            }
        }
    }
}

/// Powerful numbers that are not perfect powers.
/// 72, 108, 200, 288, 392, 432, 500, 648, 675, 800...
pub struct Achilles {
    ctr: u64,
}

impl Achilles {
    pub fn new() -> Self {
        Self { ctr: 71 }
    }
}

impl Iterator for Achilles {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.ctr = self.ctr.checked_add(1)?;
            let n = self.ctr;
            let primes = prime_factorization(self.ctr);
            if primes.iter().map(|p| p.1).all(|p| p >= 2) {
                if primes
                    .iter()
                    .map(|p| p.1)
                    .reduce(|acc, x| gcd(acc, x))
                    .unwrap()
                    == 1
                {
                    return Some(n);
                }
            }
        }
    }
}

crate::check_sequences!(
    Powerful::new(), 0, 20, [1, 4, 8, 9, 16, 25, 27, 32, 36, 49, 64, 72, 81, 100, 108, 121, 125, 128, 144, 169];
    Achilles::new(), 0, 10, [72, 108, 200, 288, 392, 432, 500, 648, 675, 800];
);
