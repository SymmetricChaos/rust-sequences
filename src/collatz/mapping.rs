use crate::{
    Number,
    core::traits::Increment,
    utils::collatz::{collatz, reduced_collatz},
};
use num::{BigInt, Integer, Zero};

/// The mapping of the Collatz function.
///
/// ```text
/// f(n) = 3n+1 for odd n
///      = n/2  for even n
///
/// 0, 4, 1, 10, 2, 16, 3, 22, 4, 28, 5, 34, 6, 40, 7, 46, 8, 52, 9, 58...
/// ```
pub struct CollatzMap<T> {
    ctr: T,
    reduced: bool,
}

impl CollatzMap<Number> {
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
impl CollatzMap<BigInt> {
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

impl Iterator for CollatzMap<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        if self.reduced {
            let out = reduced_collatz(self.ctr);
            self.ctr.incr()?;
            out
        } else {
            let out = collatz(self.ctr);
            self.ctr.incr()?;
            out
        }
    }
}

#[cfg(feature = "big_int")]
impl Iterator for CollatzMap<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        if self.reduced {
            if self.ctr.is_zero() {
                self.ctr.incr()?;
                return Some(BigInt::zero());
            }
            let mut out = self.ctr.clone();
            if out.is_odd() {
                out = (&out * 3) + 1;
            }
            while out.is_even() {
                out /= 2;
            }
            self.ctr.incr()?;
            Some(out)
        } else {
            let out = {
                if self.ctr.is_even() {
                    &self.ctr / 2
                } else {
                    (&self.ctr * 3) + 1
                }
            };
            self.ctr.incr()?;
            Some(out)
        }
    }
}

crate::check_sequences!(
    CollatzMap::new_big(), [0, 4, 1, 10, 2, 16, 3, 22, 4, 28, 5, 34, 6, 40, 7, 46, 8, 52, 9, 58, 10, 64, 11, 70, 12, 76, 13, 82, 14, 88, 15, 94, 16, 100, 17, 106, 18, 112, 19, 118, 20, 124, 21, 130, 22, 136, 23, 142, 24, 148, 25, 154, 26, 160, 27, 166, 28, 172, 29, 178, 30, 184, 31, 190, 32, 196, 33];
    CollatzMap::reduced(), [0, 1, 1, 5, 1, 1, 3, 11, 1, 7, 5, 17, 3, 5, 7, 23, 1, 13, 9, 29, 5, 1, 11, 35, 3, 19, 13, 41, 7, 11, 15, 47, 1, 25, 17, 53, 9, 7, 19, 59, 5, 31, 21, 65, 11, 17, 23, 71, 3, 37, 25, 77, 13, 5, 27, 83, 7, 43, 29, 89, 15, 23, 31, 95, 1, 49, 33, 101, 17, 13, 35, 107, 9, 55, 37, 113, 19, 29];
);

crate::sample_sequences!(
    CollatzMap::new_big();
    CollatzMap::reduced();
);
