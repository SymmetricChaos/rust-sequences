use crate::{core::traits::Increment, utils::divisibility::prime_factorization};

/// Sum of all prime factors (with multiplicity) of each natural number, starting at 1.
///
/// 0, 2, 3, 4, 5, 5, 7, 6, 6, 7, 11, 7, 13...
pub struct SumOfPrimeFactors {
    ctr: u64,
}

impl SumOfPrimeFactors {
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for SumOfPrimeFactors {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.incr()?;
        let factorization = prime_factorization(self.ctr);
        let mut sum = 0;
        for f in factorization {
            sum += f.0 * f.1
        }
        Some(sum)
    }
}

/// Sum of the distinct prime factors of each natural number, starting from 1.
///
/// 0, 2, 3, 2, 5, 5, 7, 2, 3, 7, 11, 5, 13...
pub struct SumOfDistinctPrimeFactors {
    ctr: u64,
}

impl SumOfDistinctPrimeFactors {
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for SumOfDistinctPrimeFactors {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.incr()?;
        let factorization = prime_factorization(self.ctr);
        let mut sum = 0;
        for f in factorization {
            sum += f.0
        }
        Some(sum)
    }
}

crate::check_sequences!(
    SumOfPrimeFactors::new(),         [0, 2, 3, 4, 5, 5, 7, 6, 6, 7, 11, 7, 13, 9, 8, 8, 17, 8, 19, 9, 10, 13, 23, 9, 10, 15, 9, 11, 29, 10, 31, 10, 14, 19, 12, 10, 37, 21, 16, 11, 41, 12, 43, 15, 11, 25, 47, 11, 14, 12, 20, 17, 53, 11, 16, 13, 22, 31, 59, 12, 61, 33, 13, 12, 18, 16, 67, 21, 26, 14, 71, 12, 73, 39, 13, 23, 18, 18];
    SumOfDistinctPrimeFactors::new(), [0, 2, 3, 2, 5, 5, 7, 2, 3, 7, 11, 5, 13, 9, 8, 2, 17, 5, 19, 7, 10, 13, 23, 5, 5, 15, 3, 9, 29, 10, 31, 2, 14, 19, 12, 5, 37, 21, 16, 7, 41, 12, 43, 13, 8, 25, 47, 5, 7, 7, 20, 15, 53, 5, 16, 9, 22, 31, 59, 10, 61, 33, 10, 2, 18, 16, 67, 19, 26, 14, 71, 5, 73];
);
