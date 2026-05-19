use crate::{
    core::traits::Increment,
    utils::collatz::{collatz, reduced_collatz},
};
use num::{BigInt, CheckedAdd, CheckedMul, Integer};

/// Number of steps to reach 1 for a Collatz sequence starting at each positive natural number. It is not known if this sequence is defined for all inputs.
///
/// 0, 1, 7, 2, 5, 8, 16, 3, 19, 6, 14, 9, 9...
pub struct CollatzLength<T> {
    ctr: T,
}

impl<T: Clone + Integer + CheckedAdd + CheckedMul> CollatzLength<T> {
    pub fn new() -> Self {
        Self { ctr: T::zero() }
    }
}

impl CollatzLength<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Clone + Integer + CheckedAdd + CheckedMul> Iterator for CollatzLength<T> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.incr()?;
        let mut steps: u64 = 0;

        let mut val = self.ctr.clone();
        while !val.is_one() {
            val = collatz(val)?;
            steps.incr()?;
        }

        Some(steps)
    }
}

/// Number of steps to reach 1 for the reduced Collatz sequence starting at each positive natural number. It is not known if this sequence is defined for all inputs.
///
/// 0, 1, 7, 2, 5, 8, 16, 3, 19, 6, 14, 9, 9...
pub struct ReducedCollatzLength<T> {
    ctr: T,
}

impl<T: Clone + Integer + CheckedAdd + CheckedMul> ReducedCollatzLength<T> {
    pub fn new() -> Self {
        Self { ctr: T::zero() }
    }
}

impl ReducedCollatzLength<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Clone + Integer + CheckedAdd + CheckedMul> Iterator for ReducedCollatzLength<T> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.incr()?;
        let mut steps: u64 = 0;

        let mut val = self.ctr.clone();
        while !val.is_one() {
            val = reduced_collatz(val)?;
            steps.incr()?;
        }

        Some(steps)
    }
}

crate::check_sequences!(
    CollatzLength::new_big(), [0, 1, 7, 2, 5, 8, 16, 3, 19, 6, 14, 9, 9, 17, 17, 4, 12, 20, 20, 7, 7, 15, 15, 10, 23, 10, 111, 18, 18, 18, 106, 5, 26, 13, 13, 21, 21, 21, 34, 8, 109, 8, 29, 16, 16, 16, 104, 11, 24, 24, 24, 11, 11, 112, 112, 19, 32, 19, 32, 19, 19, 107, 107, 6, 27, 27, 27, 14, 14, 14, 102, 22];
    ReducedCollatzLength::new_big(), [0, 1, 2, 1, 1, 3, 5, 1, 6, 2, 4, 3, 2, 6, 5, 1, 3, 7, 6, 2, 1, 5, 4, 3, 7, 3, 41, 6, 5, 6, 39, 1, 8, 4, 3, 7, 6, 7, 11, 2, 40, 2, 9, 5, 4, 5, 38, 3, 7, 8, 7, 3, 2, 42, 41, 6, 10, 6, 10, 6, 5, 40, 39, 1, 8, 9, 8, 4, 3, 4, 37, 7, 42, 7, 3, 7, 6, 12, 11, 2, 6, 41, 40, 2, 1, 10, 9, 5, 9, 5, 33, 5, 4, 39, 38, 3, 43, 8, 7, 8, 7, 8, 31, 3, 12, 3, 36, 42, 41, 42, 24];
);
