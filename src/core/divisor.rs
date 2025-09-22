use crate::core::{
    aliquot_sum,
    primality_utils::{number_of_divisors, prime_factorization},
    sum_of_divisors,
};

/// Number of divisors for each positive integer.
/// 1, 2, 2, 3, 2, 4, 2, 4, 3, 4...
pub struct NumberOfDivisors {
    ctr: u32,
}

impl NumberOfDivisors {
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for NumberOfDivisors {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr += 1;
        Some(number_of_divisors(self.ctr))
    }
}

/// Sum of divisors of each positive integer.
pub struct SumOfDivisors {
    ctr: u32,
}

impl SumOfDivisors {
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for SumOfDivisors {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr += 1;
        sum_of_divisors(self.ctr)
    }
}

/// The aliquot sum of  each positive integer.
pub struct AliquotSums {
    ctr: u32,
}

impl AliquotSums {
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for AliquotSums {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr += 1;
        aliquot_sum(self.ctr)
    }
}

/// The aliquot sequence starting from n.
pub struct AliquotSequence {
    ctr: u32,
}

impl AliquotSequence {
    pub fn new(n: u32) -> Self {
        Self { ctr: n }
    }
}

impl Iterator for AliquotSequence {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.ctr;
        self.ctr = aliquot_sum(self.ctr)?;
        Some(out)
    }
}

crate::print_values!(
    NumberOfDivisors::new(), 0, 10;
    SumOfDivisors::new(), 0, 10;
    AliquotSums::new(), 0, 10;
    AliquotSequence::new(10), 0, 10;
);
