use crate::{
    Number,
    utils::collatz::{collatz, reduced_collatz},
};
use num::{BigInt, Integer};

/// The values of a Collatz sequence.
///
/// ```text
/// f(n) = 3n+1 for odd n
///      = n/2  for even n
///
/// n = 19
/// 19, 58, 29, 88, 44, 22, 11, 34, 17, 52, 26, 13, 40, 20, 10, 5, 16...
///
/// n = 27
/// 27, 82, 41, 124, 62, 31, 94, 47, 142, 71, 214, 107, 322, 161, 484...
/// ```
pub struct Collatz<T> {
    n: T,
    reduced: bool,
}

impl Collatz<Number> {
    pub fn new(n: Number) -> Self {
        Self { n, reduced: false }
    }

    /// Returns only the odd values of the sequence.
    pub fn reduced(n: Number) -> Self {
        Self { n, reduced: true }
    }
}

#[cfg(feature = "big_int")]
impl Collatz<BigInt> {
    pub fn new_big<T>(n: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            n: BigInt::from(n),
            reduced: false,
        }
    }

    pub fn reduced_big<T>(n: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            n: BigInt::from(n),
            reduced: true,
        }
    }
}

impl Iterator for Collatz<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.n.clone();
        if self.reduced {
            self.n = match reduced_collatz(self.n) {
                Some(n) => n,
                None => return Some(out),
            };
        } else {
            self.n = match collatz(self.n) {
                Some(n) => n,
                None => return Some(out),
            };
        }
        Some(out)
    }
}

#[cfg(feature = "big_int")]
impl Iterator for Collatz<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.n.clone();
        if self.reduced {
            if self.n.is_odd() {
                self.n = (&self.n * 3) + 1
            }
            while self.n.is_even() {
                self.n /= 2
            }
        } else {
            if self.n.is_even() {
                self.n /= 2
            } else {
                self.n = (&self.n * 3) + 1
            }
        }

        Some(out)
    }
}

crate::check_sequences!(
    Collatz::new_big(19), [19, 58, 29, 88, 44, 22, 11, 34, 17, 52];
    Collatz::new_big(27), [27, 82, 41, 124, 62, 31, 94, 47, 142, 71];
    Collatz::reduced_big(19), [19, 29, 11, 17, 13, 5, 1, 1, 1, 1];
    Collatz::reduced_big(27), [27, 41, 31, 47, 71, 107, 161, 121, 91, 137];
    Collatz::reduced(27), [27, 41, 31, 47, 71, 107, 161, 121, 91, 137];
    Collatz::new_big(-5), [-5, -14, -7, -20, -10, -5, -14, -7, -20, -10];
);

crate::sample_sequences!(
    Collatz::new_big(19);
    Collatz::new_big(27);
    Collatz::reduced_big(19);
    Collatz::reduced_big(27);
);
