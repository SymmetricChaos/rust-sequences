use crate::{
    core::traits::Increment,
    utils::divisibility::{aliquot_sum, number_of_divisors, sum_of_divisors},
};

/// Number of divisors for each positive integer.
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
        self.ctr = self.ctr.checked_add(1)?;
        Some(number_of_divisors(self.ctr))
    }
}

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
        self.ctr = self.ctr.checked_add(1)?;
        sum_of_divisors(self.ctr)
    }
}

/// The aliquot sum of each positive integer, the sum of all of its divisors except itself.
///
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

/// The refactorable or tau numbers. Positive integers which are divisible by the number of divisors they have.
///
/// 1, 2, 8, 9, 12, 18, 24, 36, 40, 56, 60...
pub struct Refactorable {
    n: u64,
}

impl Refactorable {
    /// Only u64 output is supported.
    pub fn new() -> Self {
        Self { n: 0 }
    }
}

impl Iterator for Refactorable {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.n.incr()?;
            if self.n % number_of_divisors(self.n) == 0 {
                return Some(self.n);
            }
        }
    }
}

crate::check_sequences!(
    NumberOfDivisors::new(), [1, 2, 2, 3, 2, 4, 2, 4, 3, 4];
    SumOfDivisors::new(), [1, 3, 4, 7, 6, 12, 8, 15, 13, 18];
    AliquotSums::new(), [0, 1, 1, 3, 1, 6, 1, 7, 4, 8];
    AliquotSequence::new(10), [10, 8, 7, 1, 0, 0, 0, 0, 0, 0];
    DivisorSummatory::new(), [0, 1, 3, 5, 8, 10, 14, 16, 20, 23, 27, 29, 35, 37, 41, 45, 50, 52, 58, 60, 66, 70, 74, 76, 84, 87, 91, 95, 101, 103, 111, 113, 119, 123, 127, 131, 140, 142, 146, 150, 158, 160, 168, 170, 176, 182, 186, 188, 198, 201, 207, 211, 217, 219, 227, 231, 239, 243, 247, 249];
    Refactorable::new(), [1, 2, 8, 9, 12, 18, 24, 36, 40, 56, 60, 72, 80, 84, 88, 96, 104, 108, 128, 132, 136, 152, 156, 180, 184, 204, 225, 228, 232, 240, 248, 252, 276, 288, 296, 328, 344, 348, 360, 372, 376, 384, 396, 424, 441, 444, 448, 450, 468, 472, 480, 488, 492, 504, 516, 536];
);
