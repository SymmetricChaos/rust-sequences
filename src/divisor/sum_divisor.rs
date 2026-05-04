use crate::{core::traits::Increment, utils::divisibility::sum_of_divisors};

/// Sum of divisors of each positive integer. Also known as sigma_1(n) [σ_1(n)].
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
    SumOfDivisors::new(), [1, 3, 4, 7, 6, 12, 8, 15, 13, 18, 12, 28, 14, 24, 24, 31, 18, 39, 20, 42, 32, 36, 24, 60, 31, 42, 40, 56, 30, 72, 32, 63, 48, 54, 48, 91, 38, 60, 56, 90, 42, 96, 44, 84, 78, 72, 48, 124, 57, 93, 72, 98, 54, 120, 72, 120, 80, 90, 60, 168, 62, 96, 104, 127, 84, 144, 68, 126, 96, 144];
);
