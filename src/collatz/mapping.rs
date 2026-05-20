use crate::{
    core::traits::Increment,
    utils::collatz::{collatz, reduced_collatz},
};
use num::{BigInt, CheckedAdd, CheckedMul, Integer};

/// The mapping of the Collatz function. n/2 for even n and 3n+1 for odd n.
///
/// 0, 4, 1, 10, 2, 16, 3, 22, 4...
pub struct CollatzMap<T> {
    ctr: T,
}

impl<T: Clone + Integer + CheckedAdd + CheckedMul> CollatzMap<T> {
    pub fn new() -> Self {
        Self { ctr: T::zero() }
    }
}

impl CollatzMap<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Clone + Integer + CheckedAdd + CheckedMul> Iterator for CollatzMap<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = collatz(self.ctr.clone());
        self.ctr.incr()?;
        out
    }
}

/// The mapping of the reduced Collatz function. n/2^k for even n and (3n+1)/2^k for odd n, for the largest value of k that gives an integer.
///
/// 0, 1, 1, 5, 1, 1, 3, 11, 1, 7...
pub struct ReducedCollatzMap<T> {
    ctr: T,
}

impl<T: Clone + Integer + CheckedAdd + CheckedMul> ReducedCollatzMap<T> {
    pub fn new() -> Self {
        Self { ctr: T::zero() }
    }
}

impl ReducedCollatzMap<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Clone + Integer + CheckedAdd + CheckedMul> Iterator for ReducedCollatzMap<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ctr.is_zero() {
            self.ctr.incr()?;
            return Some(T::zero());
        }
        let out = reduced_collatz(self.ctr.clone());
        self.ctr.incr()?;
        out
    }
}

crate::check_sequences!(
    CollatzMap::new_big(), [0, 4, 1, 10, 2, 16, 3, 22, 4, 28, 5, 34, 6, 40, 7, 46, 8, 52, 9, 58, 10, 64, 11, 70, 12, 76, 13, 82, 14, 88, 15, 94, 16, 100, 17, 106, 18, 112, 19, 118, 20, 124, 21, 130, 22, 136, 23, 142, 24, 148, 25, 154, 26, 160, 27, 166, 28, 172, 29, 178, 30, 184, 31, 190, 32, 196, 33];
    ReducedCollatzMap::new_big(), [0, 1, 1, 5, 1, 1, 3, 11, 1, 7, 5, 17, 3, 5, 7, 23, 1, 13, 9, 29, 5, 1, 11, 35, 3, 19, 13, 41, 7, 11, 15, 47, 1, 25, 17, 53, 9, 7, 19, 59, 5, 31, 21, 65, 11, 17, 23, 71, 3, 37, 25, 77, 13, 5, 27, 83, 7, 43, 29, 89, 15, 23, 31, 95, 1, 49, 33, 101, 17, 13, 35, 107, 9, 55, 37, 113, 19, 29];
);
