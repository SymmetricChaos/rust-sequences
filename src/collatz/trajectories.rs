use crate::{Number, core::traits::Increment, utils::collatz::collatz};
use num::{BigInt, Integer, One, Zero};

/// The trajectory of each Collatz sequence.
///
/// ```text
/// [1], [2, 1], [3, 10, 5, 16, 8, 4, 2, 1], [4, 2, 1]...
///
/// flattened
/// 1, 2, 1, 3, 10, 5, 16, 8, 4, 2, 1, 4, 2, 1, 5, 16, 8, 4, 2, 1, 6, 3...
/// ```
pub struct CollatzTrajectories<T> {
    ctr: T,
}

impl CollatzTrajectories<Number> {
    pub fn new() -> Self {
        Self { ctr: 0 }
    }

    pub fn flattened() -> impl Iterator<Item = Number> {
        Self::new().flatten()
    }
}

#[cfg(feature = "big_int")]
impl CollatzTrajectories<BigInt> {
    pub fn new_big() -> Self {
        Self {
            ctr: BigInt::zero(),
        }
    }

    pub fn flattened_big() -> impl Iterator<Item = BigInt> {
        Self::new_big().flatten()
    }
}

impl Iterator for CollatzTrajectories<Number> {
    type Item = Vec<Number>;

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

#[cfg(feature = "big_int")]
impl Iterator for CollatzTrajectories<BigInt> {
    type Item = Vec<BigInt>;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.incr()?;

        let mut n = self.ctr.clone();
        let mut out = vec![];
        out.push(n.clone());
        // this stopping condition is not currently known to be correct for all inputs but it is correct up to huge starting values
        while !n.is_one() {
            if n.is_even() {
                n /= 2
            } else {
                n = (&n * 3) + 1
            }
            out.push(n.clone());
        }

        Some(out)
    }
}

crate::check_sequences!(
    CollatzTrajectories::flattened(), [1, 2, 1, 3, 10, 5, 16, 8, 4, 2, 1, 4, 2, 1, 5, 16, 8, 4, 2, 1, 6, 3, 10, 5, 16, 8, 4, 2, 1, 7, 22, 11, 34, 17, 52, 26, 13, 40, 20, 10, 5, 16, 8, 4, 2, 1, 8, 4, 2, 1, 9, 28, 14, 7, 22, 11, 34, 17, 52, 26, 13, 40, 20, 10, 5, 16, 8, 4, 2, 1, 10, 5, 16, 8, 4, 2, 1, 11, 34, 17, 52, 26, 13];
);

crate::sample_sequences!(
    CollatzTrajectories::new().map(|x| format!("{:?}",x));
    CollatzTrajectories::flattened();
);
