use crate::{Number, core::traits::Increment, utils::divisibility::number_of_divisors};
use num::Zero;

/// The refactorable or tau numbers. Positive integers which are divisible by the number of divisors they have.
///
/// ```text
/// 1, 2, 8, 9, 12, 18, 24, 36, 40, 56, 60, 72, 80, 84, 88, 96, 104...
/// ```
pub struct Refactorable {
    n: Number,
}

impl Refactorable {
    pub fn new() -> Self {
        Self { n: 0 }
    }
}

impl Iterator for Refactorable {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.n.incr()?;
            if (self.n % number_of_divisors(self.n)).is_zero() {
                return Some(self.n);
            }
        }
    }
}

crate::check_sequences!(
     Refactorable::new(), [1, 2, 8, 9, 12, 18, 24, 36, 40, 56, 60, 72, 80, 84, 88, 96, 104, 108, 128, 132, 136, 152, 156, 180, 184, 204, 225, 228, 232, 240, 248, 252, 276, 288, 296, 328, 344, 348, 360, 372, 376, 384, 396, 424, 441, 444, 448, 450, 468, 472, 480, 488, 492, 504, 516, 536];
);

crate::sample_sequences!(
    Refactorable::new();
);
