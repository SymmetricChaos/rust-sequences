use crate::utils::divisibility::prime_factorization;

/// The number of prime factors of each positive integer, counted with multiplicity. The prime Ω (big omega) function.
///
/// 0, 1, 1, 2, 1, 2, 1, 3, 2, 2, 1, 3, 1, 2, 2...
pub struct NumberOfPrimeFactors {
    ctr: u64,
}

impl NumberOfPrimeFactors {
    /// Only u64 output is supported
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for NumberOfPrimeFactors {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr = self.ctr.checked_add(1)?;

        Some(prime_factorization(self.ctr).iter().map(|x| x.1).sum())
    }
}

/// The number of distinct prime factors of each positive integer. The prime ω (small omega) function.
///
/// 0, 1, 1, 1, 1, 2, 1, 1, 1, 2, 1, 2, 1, 2...
pub struct NumberOfDistinctPrimeFactors {
    ctr: u64,
}

impl NumberOfDistinctPrimeFactors {
    /// Only u64 output is supported
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for NumberOfDistinctPrimeFactors {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr = self.ctr.checked_add(1)?;

        Some(prime_factorization(self.ctr).len() as u64)
    }
}

crate::check_sequences!(
    NumberOfPrimeFactors::new(), [0, 1, 1, 2, 1, 2, 1, 3, 2, 2, 1, 3, 1, 2, 2, 4, 1, 3, 1, 3, 2, 2, 1, 4, 2, 2, 3, 3, 1, 3, 1, 5, 2, 2, 2, 4, 1, 2, 2, 4, 1, 3, 1, 3, 3, 2, 1, 5, 2, 3, 2, 3, 1, 4, 2, 4, 2, 2, 1, 4, 1, 2, 3, 6, 2, 3, 1, 3, 2, 3, 1, 5, 1, 2, 3, 3, 2, 3, 1, 5, 4, 2, 1, 4, 2, 2, 2, 4, 1, 4, 2, 3, 2, 2, 2, 6, 1, 3, 3, 4, 1, 3, 1, 4, 3, 2, 1, 5, 1, 3, 2];
    NumberOfDistinctPrimeFactors::new(), [0, 1, 1, 1, 1, 2, 1, 1, 1, 2, 1, 2, 1, 2, 2, 1, 1, 2, 1, 2, 2, 2, 1, 2, 1, 2, 1, 2, 1, 3, 1, 1, 2, 2, 2, 2, 1, 2, 2, 2, 1, 3, 1, 2, 2, 2, 1, 2, 1, 2, 2, 2, 1, 2, 2, 2, 2, 2, 1, 3, 1, 2, 2, 1, 2, 3, 1, 2, 2, 3, 1, 2, 1, 2, 2, 2, 2, 3, 1, 2, 1, 2, 1, 3, 2, 2, 2, 2, 1, 3, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 1, 3, 1, 2, 3, 2, 1, 2, 1, 3, 2];
);
