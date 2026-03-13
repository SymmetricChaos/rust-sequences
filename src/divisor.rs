use crate::utils::divisibility::{aliquot_sum, number_of_divisors, sum_of_divisors};

/// Number of divisors for each positive integer.
/// 1, 2, 2, 3, 2, 4, 2, 4, 3, 4...
pub struct NumberOfDivisors {
    ctr: u64,
}

impl NumberOfDivisors {
    /// Only u64 output is supported.
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for NumberOfDivisors {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr = self.ctr.checked_add(1)?;
        Some(number_of_divisors(self.ctr))
    }
}

/// Sum of divisors of each positive integer.
/// 1, 3, 4, 7, 6, 12, 8, 15, 13, 18...
pub struct SumOfDivisors {
    ctr: u64,
}

impl SumOfDivisors {
    /// Only u64 output is supported.
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for SumOfDivisors {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr = self.ctr.checked_add(1)?;
        sum_of_divisors(self.ctr)
    }
}

/// The aliquot sum of each positive integer, the sum of all of its divisors except itself.
/// 0, 1, 1, 3, 1, 6, 1, 7, 4, 8...
pub struct AliquotSums {
    ctr: u64,
}

impl AliquotSums {
    /// Only u64 output is supported.
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for AliquotSums {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr = self.ctr.checked_add(1)?;
        aliquot_sum(self.ctr)
    }
}

/// The aliquot sequence starting from n. Each term is the aliquot sum of the previous.
pub struct AliquotSequence {
    n: u64,
}

impl AliquotSequence {
    /// Only u64 output is supported.
    pub fn new(n: u64) -> Self {
        Self { n }
    }
}

impl Iterator for AliquotSequence {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.n;
        self.n = aliquot_sum(self.n)?;
        Some(out)
    }
}

crate::check_sequences!(
    NumberOfDivisors::new(), [1, 2, 2, 3, 2, 4, 2, 4, 3, 4];
    SumOfDivisors::new(), [1, 3, 4, 7, 6, 12, 8, 15, 13, 18];
    AliquotSums::new(), [0, 1, 1, 3, 1, 6, 1, 7, 4, 8];
    AliquotSequence::new(10), [10, 8, 7, 1, 0, 0, 0, 0, 0, 0];
);
