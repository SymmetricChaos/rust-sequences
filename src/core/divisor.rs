use crate::core::{
    aliquot_sum, cototient, primality_utils::number_of_divisors, sum_of_divisors, totient,
};

/// Number of divisors for each positive integer.
/// 1, 2, 2, 3, 2, 4, 2, 4, 3, 4...
pub struct NumberOfDivisors {
    ctr: u64,
}

impl NumberOfDivisors {
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
pub struct SumOfDivisors {
    ctr: u64,
}

impl SumOfDivisors {
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

/// The aliquot sum of  each positive integer.
pub struct AliquotSums {
    ctr: u64,
}

impl AliquotSums {
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

/// The aliquot sequence starting from n.
pub struct AliquotSequence {
    n: u64,
}

impl AliquotSequence {
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

/// The totient of each positive integer.
pub struct Totients {
    ctr: u64,
}

impl Totients {
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for Totients {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr = self.ctr.checked_add(1)?;
        Some(totient(self.ctr))
    }
}

/// The cototient of each positive integer.
pub struct Cototients {
    ctr: u64,
}

impl Cototients {
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

crate::print_values!(
    NumberOfDivisors::new(), 0, 10;
    SumOfDivisors::new(), 0, 10;
    AliquotSums::new(), 0, 10;
    AliquotSequence::new(10), 0, 10;
    Totients::new(), 0, 10;
    Cototients::new(), 0, 10;
);

crate::check_sequences!(
    NumberOfDivisors::new(), 0, 10, [1, 2, 2, 3, 2, 4, 2, 4, 3, 4];
    SumOfDivisors::new(), 0, 10, [1, 3, 4, 7, 6, 12, 8, 15, 13, 18];
    AliquotSums::new(), 0, 10, [0, 1, 1, 3, 1, 6, 1, 7, 4, 8];
    AliquotSequence::new(10), 0, 10, [10, 8, 7, 1, 0, 0, 0, 0, 0, 0];
    Totients::new(), 0, 10, [1, 1, 2, 2, 4, 2, 6, 4, 6, 4];
    Cototients::new(), 0, 10, [0, 1, 1, 2, 1, 4, 1, 4, 3, 6];
);
