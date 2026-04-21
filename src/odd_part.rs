use crate::core::traits::Increment;
use num::{BigInt, CheckedAdd, CheckedDiv, Integer};

/// The odd part of each positive integer. The value after dividing by the largest power of two that is a factor.
///
/// 1, 1, 3, 1, 5, 3, 7, 1, 9, 5...
pub struct OddPart<T> {
    ctr: T,
    two: T,
}

impl<T: Clone + Integer> OddPart<T> {
    pub fn new() -> Self {
        Self {
            ctr: T::zero(),
            two: T::one() + T::one(),
        }
    }
}

impl OddPart<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Clone + Integer + CheckedDiv + CheckedAdd> Iterator for OddPart<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.incr()?;

        let mut n = self.ctr.clone();

        while n.is_even() {
            n = n.checked_div(&self.two).unwrap(); // Can't fail but allows using a reference to divide
        }

        Some(n)
    }
}

crate::check_sequences!(
    OddPart::<i8>::new(), [1, 1, 3, 1, 5, 3, 7, 1, 9, 5];
);
