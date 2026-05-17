use num::{BigInt, CheckedAdd, CheckedDiv, CheckedMul, Integer};

use crate::core::traits::Increment;

/// The mapping of the Collatz function. n/2 for even n and 3n+1 for odd n.
///
/// 0, 4, 1, 10, 2, 16, 3, 22, 4...
pub struct CollatzMap<T> {
    ctr: T,
}

impl<T: Clone + Integer + CheckedAdd + CheckedDiv + CheckedMul> CollatzMap<T> {
    pub fn new() -> Self {
        Self { ctr: T::zero() }
    }
}

impl CollatzMap<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Clone + Integer + CheckedAdd + CheckedDiv + CheckedMul> Iterator for CollatzMap<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = if self.ctr.is_even() {
            self.ctr.checked_div(&(T::one() + T::one()))
        } else {
            self.ctr
                .checked_mul(&(T::one() + T::one() + T::one()))?
                .checked_add(&T::one())
        };
        self.ctr.incr()?;
        out
    }
}

/// The mapping of the reduced Collatz function.
///
/// 0, 1, 1, 5, 1, 1, 3, 11, 1, 7...
pub struct ReducedCollatzMap<T> {
    ctr: T,
}

impl<T: Clone + Integer + CheckedAdd + CheckedDiv + CheckedMul> ReducedCollatzMap<T> {
    pub fn new() -> Self {
        Self { ctr: T::zero() }
    }
}

impl ReducedCollatzMap<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Clone + Integer + CheckedAdd + CheckedDiv + CheckedMul> Iterator for ReducedCollatzMap<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ctr.is_zero() {
            self.ctr.incr()?;
            return Some(T::zero());
        }
        let mut n = self.ctr.clone();
        self.ctr.incr()?;
        if n.is_odd() {
            n = n
                .checked_mul(&(T::one() + T::one() + T::one()))?
                .checked_add(&T::one())?;
        };
        while n.is_even() {
            n = n.checked_div(&(T::one() + T::one()))?;
        }
        Some(n)
    }
}

crate::check_sequences!(
    CollatzMap::new_big(), [0, 4, 1, 10, 2, 16, 3, 22, 4, 28, 5, 34, 6, 40, 7, 46, 8, 52, 9, 58, 10, 64, 11, 70, 12, 76, 13, 82, 14, 88, 15, 94, 16, 100, 17, 106, 18, 112, 19, 118, 20, 124, 21, 130, 22, 136, 23, 142, 24, 148, 25, 154, 26, 160, 27, 166, 28, 172, 29, 178, 30, 184, 31, 190, 32, 196, 33];
    ReducedCollatzMap::new_big(), [0, 1, 1, 5, 1, 1, 3, 11, 1, 7];
);
