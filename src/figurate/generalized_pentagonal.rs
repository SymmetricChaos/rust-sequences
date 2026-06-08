use num::{BigInt, CheckedAdd, CheckedMul, CheckedSub, Signed};

use crate::{Number, core::integer::Integers};

/// The generalized pentagonal numbers. Extends the domain of the pentagonal numbers to all integers.
///
/// ```text
/// 0, 1, 2, 5, 7, 12, 15, 22, 26, 35, 40, 51, 57, 70, 77, 92, 100, 117...
/// ```
pub struct GeneralizedPentagonal<T> {
    integers: Integers<T>,
    three: T,
}

impl GeneralizedPentagonal<Number> {
    pub fn new() -> Self {
        Self {
            three: 3,
            integers: Integers::new(),
        }
    }
}

#[cfg(feature = "big_int")]
impl GeneralizedPentagonal<BigInt> {
    pub fn new_big() -> Self {
        Self {
            three: BigInt::from(3),
            integers: Integers::new_big(),
        }
    }
}

impl<T: Signed + CheckedAdd + Clone + CheckedMul + CheckedSub + std::ops::Shr<i32, Output = T>>
    Iterator for GeneralizedPentagonal<T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.integers.next()?;
        let out = self
            .three
            .checked_mul(&n)?
            .checked_mul(&n)?
            .checked_sub(&n)?
            >> 1;
        Some(out)
    }
}

crate::check_iteration_times!(
    GeneralizedPentagonal::new_big(), 1_000_000;
    GeneralizedPentagonal::new(), 1_000_000;
);

crate::check_sequences!(
    GeneralizedPentagonal::new_big(), [0, 1, 2, 5, 7, 12, 15, 22, 26, 35, 40, 51, 57, 70, 77, 92, 100, 117, 126, 145, 155, 176, 187, 210, 222, 247, 260, 287, 301, 330, 345, 376, 392, 425, 442, 477, 495, 532, 551, 590, 610, 651, 672, 715, 737, 782, 805, 852, 876, 925, 950, 1001, 1027, 1080, 1107, 1162, 1190, 1247, 1276, 1335];
);

crate::sample_sequences!(
    GeneralizedPentagonal::new();
);
