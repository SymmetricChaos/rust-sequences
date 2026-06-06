use crate::utils::divisibility::prime_factorization;
use crate::{Number, core::traits::Increment};
use num::{BigInt, Integer, Zero};

/// The Möbius function over the positive integers.
///
/// ```text
/// 1, -1, -1, 0, -1, 1, -1, 0, 0, 1, -1, 0, -1, 1, 1, 0, -1, 0, -1, 0...
/// ```
pub struct Mobius<T> {
    n: T,
}

impl Mobius<Number> {
    pub fn new() -> Self {
        Self { n: 0 }
    }
}

impl Iterator for Mobius<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        self.n.incr()?;

        if self.n == 1 {
            return Some(1);
        } else {
            let pf = prime_factorization(self.n);
            if pf.iter().map(|x| x.1).any(|m| m > 1) {
                return Some(0);
            } else {
                if pf.len().is_even() {
                    return Some(1);
                } else {
                    return Some(-1);
                }
            }
        }
    }
}

/// The values of the Merterns function over the positive integers. Partial sums of the Mobius function, without the leading zero.
///
/// 1, 0, -1, -1, -2, -1, -2, -2, -2, -1, -2, -2, -3, -2, -1, -1, -2...
pub struct Mertens<T> {
    n: Number,
    sum: T,
}

impl Mertens<Number> {
    pub fn new() -> Self {
        Self { n: 0, sum: 0 }
    }
}

impl Mertens<BigInt> {
    pub fn new_big() -> Self {
        Self {
            n: 0,
            sum: BigInt::zero(),
        }
    }
}

impl Iterator for Mertens<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        self.n.incr()?;

        if self.n == 1 {
            self.sum = self.sum.checked_add(1)?;
        } else {
            let pf = prime_factorization(self.n);
            if pf.iter().map(|x| x.1).any(|m| m > 1) {
                // add zero
            } else {
                if pf.len().is_even() {
                    self.sum = self.sum.checked_add(1)?;
                } else {
                    self.sum = self.sum.checked_add(-1)?;
                }
            }
        }

        Some(self.sum.clone())
    }
}

crate::check_sequences!(
    Mobius::new(), [1, -1, -1, 0, -1, 1, -1, 0, 0, 1, -1, 0, -1, 1, 1, 0, -1, 0, -1, 0];
    Mertens::new(), [1, 0, -1, -1, -2, -1, -2, -2, -2, -1, -2, -2, -3, -2, -1, -1, -2, -2, -3, -3];
);

crate::sample_sequences!(
    Mobius::new();
    Mertens::new();
);
