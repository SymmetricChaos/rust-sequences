use num::{BigInt, CheckedAdd, Integer};
use std::ops::{BitXor, Shr};

/// The Gray codes, bit sequence where each term differs from the previous by a single bit. A permutation of the integers.
pub struct Gray<T> {
    ctr: T,
}

impl<T: Shr<usize, Output = T> + Clone + Integer + CheckedAdd + BitXor<Output = T>> Gray<T> {
    pub fn new() -> Self {
        Self { ctr: T::zero() }
    }
}

impl Gray<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Clone + Integer + CheckedAdd + BitXor<Output = T> + Shr<usize, Output = T>> Iterator
    for Gray<T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.ctr.clone() ^ (self.ctr.clone() >> 1);
        self.ctr = self.ctr.checked_add(&T::one())?;
        Some(out)
    }
}

crate::check_sequences!(
    Gray::new_big(), [0, 1, 3, 2, 6, 7, 5, 4, 12, 13];
    Gray::<i8>::new(), [0, 1, 3, 2, 6, 7, 5, 4, 12, 13];
);
