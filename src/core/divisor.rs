use crate::core::utils::number_of_divisors;

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
        Some(todo!())
    }
}

crate::print_values!(
    NumberOfDivisors::new(), 0, 10;
);
