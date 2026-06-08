use crate::{Number, core::traits::Increment, utils::collatz::collatz};
use num::{BigInt, Integer, One, Zero};

/// The maximum value reached by the Collatz sequence that starts at each positive integer.
///
/// ```text
/// 1, 2, 16, 4, 16, 16, 52, 8, 52, 16, 52, 16, 40, 52, 160, 16, 52, 52...
/// ```
pub struct CollatzHeights<T> {
    ctr: T,
}

impl CollatzHeights<Number> {
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl CollatzHeights<BigInt> {
    pub fn new_big() -> Self {
        Self {
            ctr: BigInt::zero(),
        }
    }
}

impl Iterator for CollatzHeights<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.incr()?;

        let mut peak = self.ctr.clone();
        let mut val = self.ctr.clone();

        while !val.is_one() {
            val = collatz(val)?;

            if val > peak {
                peak = val.clone()
            }
        }

        Some(peak)
    }
}

impl Iterator for CollatzHeights<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.inc();

        let mut peak = self.ctr.clone();
        let mut val = self.ctr.clone();

        while !val.is_one() {
            if val.is_even() {
                val /= 2
            } else {
                val = (&val * 3) + 1
            }

            if val > peak {
                peak = val.clone()
            }
        }

        Some(peak)
    }
}

crate::check_sequences!(
    CollatzHeights::new_big(), [1, 2, 16, 4, 16, 16, 52, 8, 52, 16, 52, 16, 40, 52, 160, 16, 52, 52, 88, 20, 64, 52, 160, 24, 88, 40, 9232, 52, 88, 160, 9232, 32, 100, 52, 160, 52, 112, 88, 304, 40, 9232, 64, 196, 52, 136, 160, 9232, 48, 148, 88, 232, 52, 160, 9232, 9232, 56, 196, 88, 304, 160, 184, 9232];
);

crate::sample_sequences!(
    CollatzHeights::new_big();
);
