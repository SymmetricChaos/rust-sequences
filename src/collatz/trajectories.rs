use crate::{core::traits::Increment, utils::collatz::collatz};
use num::{BigInt, CheckedAdd, CheckedMul, Integer, Zero};

/// The trajectory of each Collatz sequence.
///
/// ```text
/// [1], [2,1], [3, 10, 5, 16, 8, 4, 2, 1]...
/// ```
pub struct CollatzTrajectories<T> {
    ctr: T,
}

impl<T: Clone + Zero> CollatzTrajectories<T> {
    pub fn new() -> Self {
        Self { ctr: T::zero() }
    }
}

impl CollatzTrajectories<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Clone + CheckedAdd + CheckedMul + Integer> Iterator for CollatzTrajectories<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.incr()?;

        let mut n = self.ctr.clone();
        let mut out = vec![];
        out.push(n.clone());
        // this stopping condition is not currently known to be correct for all inputs but it is correct up to huge starting values
        while !n.is_one() {
            n = collatz(n.clone())?;
            out.push(n.clone());
        }

        Some(out)
    }
}

crate::check_sequences!(
    CollatzTrajectories::new_big().flatten(), [1, 2, 1, 3, 10, 5, 16, 8, 4, 2, 1, 4, 2, 1, 5, 16, 8, 4, 2, 1, 6, 3, 10, 5, 16, 8, 4, 2, 1, 7, 22, 11, 34, 17, 52, 26, 13, 40, 20, 10, 5, 16, 8, 4, 2, 1, 8, 4, 2, 1, 9, 28, 14, 7, 22, 11, 34, 17, 52, 26, 13, 40, 20, 10, 5, 16, 8, 4, 2, 1, 10, 5, 16, 8, 4, 2, 1, 11, 34, 17, 52, 26, 13];
);
