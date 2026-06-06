use crate::{Number, core::traits::Increment};
use num::{BigInt, CheckedAdd, Integer, Zero};

/// The odd part of each positive integer. The value after dividing by the largest power of two that is a divisor.
///
/// ```text
/// 1, 1, 3, 1, 5, 3, 7, 1, 9, 5, 11, 3, 13, 7, 15, 1, 17, 9, 19, 5, 21...
/// ```
pub struct OddPart<T> {
    ctr: T,
}

impl OddPart<Number> {
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl OddPart<BigInt> {
    pub fn new_big() -> Self {
        Self {
            ctr: BigInt::zero(),
        }
    }
}

impl<T: Clone + CheckedAdd + Integer + std::ops::Shr<i32, Output = T>> Iterator for OddPart<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.incr()?;

        let mut n = self.ctr.clone();

        while n.is_even() {
            n = n >> 1;
        }

        Some(n)
    }
}

crate::check_sequences!(
    OddPart::new(), [1, 1, 3, 1, 5, 3, 7, 1, 9, 5, 11, 3, 13, 7, 15, 1, 17, 9, 19, 5, 21, 11, 23, 3, 25, 13, 27, 7, 29, 15, 31, 1, 33, 17, 35, 9, 37, 19, 39, 5, 41, 21, 43, 11, 45, 23, 47, 3, 49, 25, 51, 13, 53, 27, 55, 7, 57, 29, 59, 15, 61, 31, 63, 1, 65, 33, 67, 17, 69, 35, 71, 9, 73, 37, 75, 19, 77];
);

crate::sample_sequences!(
    OddPart::new();
);
