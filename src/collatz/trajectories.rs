use crate::{
    Number,
    core::traits::Increment,
    utils::collatz::{collatz, reduced_collatz},
};
use num::{BigInt, Integer, One, Zero};

/// The trajectory of each Collatz sequence.
///
/// ```text
/// [1], [2, 1], [3, 10, 5, 16, 8, 4, 2, 1], [4, 2, 1]...
/// ```
pub struct CollatzTrajectories<T> {
    ctr: T,
    reduced: bool,
}

impl CollatzTrajectories<Number> {
    pub fn new() -> Self {
        Self {
            ctr: 0,
            reduced: false,
        }
    }

    pub fn reduced() -> Self {
        Self {
            ctr: 0,
            reduced: true,
        }
    }
}

#[cfg(feature = "big_int")]
impl CollatzTrajectories<BigInt> {
    pub fn new_big() -> Self {
        Self {
            ctr: BigInt::zero(),
            reduced: false,
        }
    }

    pub fn reduced_big() -> Self {
        Self {
            ctr: BigInt::zero(),
            reduced: true,
        }
    }
}

impl Iterator for CollatzTrajectories<Number> {
    type Item = Vec<Number>;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.incr()?;

        let mut n = self.ctr.clone();
        let mut out = vec![];
        out.push(n.clone());

        if self.reduced {
            while !n.is_one() {
                n = reduced_collatz(n.clone())?;
                out.push(n.clone());
            }
        } else {
            while !n.is_one() {
                n = collatz(n.clone())?;
                out.push(n.clone());
            }
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

        if self.reduced {
            while !n.is_one() {
                if n.is_odd() {
                    n = 3 * n + 1;
                };
                while n.is_even() {
                    n = n / 2;
                }
                out.push(n.clone());
            }
        } else {
            while !n.is_one() {
                if n.is_even() {
                    n = n / 2;
                } else {
                    n = 3 * n + 1;
                }
                out.push(n.clone());
            }
        }

        Some(out)
    }
}

crate::check_sequences!(
    CollatzTrajectories::new().flatten(), [1, 2, 1, 3, 10, 5, 16, 8, 4, 2, 1, 4, 2, 1, 5, 16, 8, 4, 2, 1, 6, 3, 10, 5, 16, 8, 4, 2, 1, 7, 22, 11, 34, 17, 52, 26, 13, 40, 20, 10, 5, 16, 8, 4, 2, 1, 8, 4, 2, 1, 9, 28, 14, 7, 22, 11, 34, 17, 52, 26, 13, 40, 20, 10, 5, 16, 8, 4, 2, 1, 10, 5, 16, 8, 4, 2, 1, 11, 34, 17, 52, 26, 13];
    // CollatzTrajectories::reduced().flatten(), [0,0,0,0,0,0,0]; this doesn't have an OEIS entry
);

crate::sample_sequences!(
    CollatzTrajectories::new().map(|x| format!("{:?}",x));
    CollatzTrajectories::reduced().map(|x| format!("{:?}",x));
);
