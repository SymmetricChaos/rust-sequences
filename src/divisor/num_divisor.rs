use crate::{core::traits::Increment, utils::divisibility::number_of_divisors};

/// Number of divisors for each positive integer. Also known as sigma_0(n) [σ_0(n)].
///
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
        self.ctr.incr()?;
        Some(number_of_divisors(self.ctr))
    }
}

/// The partial sums of the divisor function over the natural numbers.
///
/// 0, 1, 3, 5, 8, 10, 14, 16, 20, 23...
pub struct DivisorSummatory {
    n: u64,
    s: u64,
}

impl DivisorSummatory {
    /// Only u64 output is supported.
    pub fn new() -> Self {
        Self { n: 0, s: 0 }
    }
}

impl Iterator for DivisorSummatory {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.s;
        self.n += 1;
        match self.s.checked_add(number_of_divisors(self.n)) {
            Some(x) => self.s = x,
            None => return Some(out),
        }
        Some(out)
    }
}

crate::check_sequences!(
    NumberOfDivisors::new(), [1, 2, 2, 3, 2, 4, 2, 4, 3, 4, 2, 6, 2, 4, 4, 5, 2, 6, 2, 6, 4, 4, 2, 8, 3, 4, 4, 6, 2, 8, 2, 6, 4, 4, 4, 9, 2, 4, 4, 8, 2, 8, 2, 6, 6, 4, 2, 10, 3, 6, 4, 6, 2, 8, 4, 8, 4, 4, 2, 12, 2, 4, 6, 7, 4, 8, 2, 6, 4, 8, 2, 12, 2, 4, 6, 6, 4, 8, 2, 10, 5, 4, 2, 12, 4, 4, 4, 8, 2, 12, 4, 6, 4, 4, 4, 12, 2, 6, 6, 9, 2, 8, 2, 8];
    DivisorSummatory::new(), [0, 1, 3, 5, 8, 10, 14, 16, 20, 23, 27, 29, 35, 37, 41, 45, 50, 52, 58, 60, 66, 70, 74, 76, 84, 87, 91, 95, 101, 103, 111, 113, 119, 123, 127, 131, 140, 142, 146, 150, 158, 160, 168, 170, 176, 182, 186, 188, 198, 201, 207, 211, 217, 219, 227, 231, 239, 243, 247, 249];
);
