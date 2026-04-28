use crate::core::traits::{CountBits, Increment};
use num::{BigInt, CheckedAdd, Integer};

/// The binary weight of each natural number, starting from zero.
///
/// 0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2...
pub struct BinaryWeight<T> {
    ctr: T,
}

impl<T: Integer + CountBits> BinaryWeight<T> {
    pub fn new() -> Self {
        Self { ctr: T::zero() }
    }
}

impl BinaryWeight<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Integer + CountBits + CheckedAdd> Iterator for BinaryWeight<T> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let n = Some(self.ctr.count_ones_64());
        self.ctr.incr()?;
        n
    }
}

crate::check_sequences!(
    BinaryWeight::<u32>::new(), [0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4, 1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 2, 3, 3, 4, 3, 4, 4, 5, 3];
    BinaryWeight::new_big(), [0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4, 1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 2, 3, 3, 4, 3, 4, 4, 5, 3];
);
