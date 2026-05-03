use crate::{
    core::traits::Increment,
    utils::divisibility::{prime_factorization, prime_signature},
};
use num::One;

/// The the prime factorization of each positive integer.
/// For instance 20 = 2^2 + 5^1 and is written here as [(2,2), (5,1)]
///
/// [], [(2, 1)], [(3, 1)], [(2, 2)], [(5, 1)], [(2, 1), (3, 1)], [(7, 1)], [(2, 3)], [(3, 2)], [(2, 1), (5, 1)]
pub struct PrimeFactorizations {
    ctr: u64,
}

impl PrimeFactorizations {
    /// Only u64 output is supported
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for PrimeFactorizations {
    type Item = Vec<(u64, u64)>;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.incr()?;

        Some(prime_factorization(self.ctr))
    }
}

/// The greatest prime factor of each positive integer. Defined as 1 for 1.
///
/// 1, 2, 3, 2, 5, 3, 7, 2, 3, 5, 11, 3, 13, 7
pub struct GreatestPrimeFactor {
    ctr: u64,
}

impl GreatestPrimeFactor {
    /// Only u64 output is supported
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for GreatestPrimeFactor {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.incr()?;

        if self.ctr.is_one() {
            return Some(1);
        }

        Some(prime_factorization(self.ctr).last()?.0)
    }
}

/// The least prime factor of each positive integer. Defined as 1 for 1.
///
/// 1, 2, 3, 2, 5, 2, 7, 2, 3, 2, 11, 2, 13, 2
pub struct LeastPrimeFactor {
    ctr: u64,
}

impl LeastPrimeFactor {
    /// Only u64 output is supported
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for LeastPrimeFactor {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.incr()?;

        if self.ctr.is_one() {
            return Some(1);
        }

        Some(prime_factorization(self.ctr)[0].0)
    }
}

/// The the prime signature of each positive integer. The powers of the prime factorization in decreasing order.
/// For instance the prime signature of 3918213 is [4, 2, 1] because 3918213 = 3^4 * 13^1 * 61^2.
///
/// [], [1], [1], [2], [1], [1, 1], [1], [3], [2], [1, 1]...
pub struct PrimeSignatures {
    ctr: u64,
}

impl PrimeSignatures {
    /// Only u64 output is supported
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for PrimeSignatures {
    type Item = Vec<u64>;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr = self.ctr.checked_add(1)?;

        Some(prime_signature(self.ctr))
    }
}

crate::print_sequences!(
    PrimeSignatures::new(), 30, "{:?}", ", ";
    PrimeFactorizations::new(), 15, "{:?}", ", ";
);

crate::check_sequences!(
    GreatestPrimeFactor::new(), [1, 2, 3, 2, 5, 3, 7, 2, 3, 5, 11, 3, 13, 7, 5, 2, 17, 3, 19, 5, 7, 11, 23, 3, 5, 13, 3, 7, 29, 5, 31, 2, 11, 17, 7, 3, 37, 19, 13, 5, 41, 7, 43, 11, 5, 23, 47, 3, 7, 5, 17, 13, 53, 3, 11, 7, 19, 29, 59, 5, 61, 31, 7, 2, 13, 11, 67, 17, 23, 7, 71, 3, 73, 37, 5, 19, 11, 13, 79, 5, 3, 41, 83, 7, 17, 43];
    LeastPrimeFactor::new(), [1, 2, 3, 2, 5, 2, 7, 2, 3, 2, 11, 2, 13, 2, 3, 2, 17, 2, 19, 2, 3, 2, 23, 2, 5, 2, 3, 2, 29, 2, 31, 2, 3, 2, 5, 2, 37, 2, 3, 2, 41, 2, 43, 2, 3, 2, 47, 2, 7, 2, 3, 2, 53, 2, 5, 2, 3, 2, 59, 2, 61, 2, 3, 2, 5, 2, 67, 2, 3, 2, 71, 2, 73, 2, 3, 2, 7, 2, 79, 2, 3, 2, 83, 2, 5, 2, 3, 2, 89, 2, 7, 2, 3, 2, 5, 2, 97];
);
