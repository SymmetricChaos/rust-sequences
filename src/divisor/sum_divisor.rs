use crate::{core::traits::Increment, utils::divisibility::sum_of_divisors};

/// Sum of divisors of each positive integer.
///
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
        self.ctr.incr()?;
        sum_of_divisors(self.ctr)
    }
}
crate::check_sequences!(
    SumOfDivisors::new(), [1, 3, 4, 7, 6, 12, 8, 15, 13, 18];
);
