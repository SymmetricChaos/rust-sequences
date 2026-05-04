use crate::utils::divisibility::prime_factorization;
use num::integer::gcd;

/// Positive natural numbers that are divisible by the squares of all their prime divisors.
///
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

/// The Achilles numbers, powerful numbers that are not perfect powers.
///
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
    Powerful::new(), [1, 4, 8, 9, 16, 25, 27, 32, 36, 49, 64, 72, 81, 100, 108, 121, 125, 128, 144, 169, 196, 200, 216, 225, 243, 256, 288, 289, 324, 343, 361, 392, 400, 432, 441, 484, 500, 512, 529, 576, 625, 648, 675, 676, 729, 784, 800, 841, 864, 900, 961, 968, 972, 1000];
    Achilles::new(), [72, 108, 200, 288, 392, 432, 500, 648, 675, 800, 864, 968, 972, 1125, 1152, 1323, 1352, 1372, 1568, 1800, 1944, 2000, 2312, 2592, 2700, 2888, 3087, 3200, 3267, 3456, 3528, 3872, 3888, 4000, 4232, 4500, 4563, 4608, 5000, 5292, 5324, 5400, 5408, 5488, 6075];
);
