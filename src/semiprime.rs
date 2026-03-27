use crate::utils::divisibility::prime_factorization;

/// The positive integers which are the product of exactly two prime numbers.
///
/// 4, 6, 9, 10, 14, 15, 21, 22, 25, 26, 33, 34, 35...
pub struct Semiprime {
    ctr: u64,
}

impl Semiprime {
    /// Omly u64 output is supported.
    pub fn new() -> Self {
        Self { ctr: 3 }
    }
}

impl Iterator for Semiprime {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.ctr += 1;
            let s: u64 = prime_factorization(self.ctr).into_iter().map(|x| x.1).sum();
            if s == 2_u64 {
                return Some(self.ctr);
            }
        }
    }
}

/// The positive integers which are the product of exactly k prime numbers.
pub struct AlmostPrime {
    ctr: u64,
    k: u64,
}

impl AlmostPrime {
    /// k must be positive.
    /// Omly u64 output is supported.
    pub fn new(k: u64) -> Self {
        assert!(k > 0);
        Self { ctr: 2, k }
    }
}

impl Iterator for AlmostPrime {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.ctr += 1;
            let s: u64 = prime_factorization(self.ctr).into_iter().map(|x| x.1).sum();
            if s == self.k {
                return Some(self.ctr);
            }
        }
    }
}

crate::check_sequences!(
    Semiprime::new(), [4, 6, 9, 10, 14, 15, 21, 22, 25, 26, 33, 34, 35];
    AlmostPrime::new(2), [4, 6, 9, 10, 14, 15, 21, 22, 25, 26, 33, 34, 35];
    AlmostPrime::new(3), [8, 12, 18, 20, 27, 28, 30];
);
