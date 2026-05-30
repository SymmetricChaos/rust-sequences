use crate::{Number, core::traits::Increment};
use num::{BigInt, Integer, Signed, Zero};

/// The Champernowne constants. Infinite words formed by listing the digits of the natural numbers in a given base.
pub struct Champernowne<T> {
    ctr: T,
    digits: Vec<T>,
    base: T,
}

impl Champernowne<Number> {
    pub fn new(base: Number) -> Self {
        Self {
            ctr: 0,
            digits: Vec::new(),
            base,
        }
    }
}

impl Champernowne<BigInt> {
    pub fn new_big<G>(base: G) -> Self
    where
        BigInt: From<G>,
    {
        Self {
            ctr: BigInt::zero(),
            digits: Vec::new(),
            base: BigInt::from(base),
        }
    }
}

impl Iterator for Champernowne<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ctr.is_zero() {
            self.ctr.incr()?;
            self.digits.push(0);
        }
        if self.digits.is_empty() {
            let mut n = self.ctr.clone();
            while n.is_positive() {
                let (div, rem) = n.div_rem(&self.base);
                self.digits.push(rem);
                n = div;
            }
            self.ctr.incr()?;
        }
        self.digits.pop()
    }
}

impl Iterator for Champernowne<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ctr.is_zero() {
            self.ctr.incr()?;
            self.digits.push(BigInt::zero());
        }
        if self.digits.is_empty() {
            let mut n = self.ctr.clone();
            while n.is_positive() {
                let (div, rem) = n.div_rem(&self.base);
                self.digits.push(rem);
                n = div;
            }
            self.ctr.incr()?;
        }
        self.digits.pop()
    }
}

crate::check_sequences!(
    Champernowne::new_big(2), [0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0];
    Champernowne::new_big(10), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 0, 1, 1, 1, 2, 1, 3, 1, 4, 1, 5, 1, 6, 1, 7, 1, 8, 1, 9, 2, 0, 2, 1, 2, 2, 2, 3, 2, 4, 2, 5, 2, 6, 2, 7, 2, 8, 2, 9, 3, 0, 3, 1, 3, 2, 3, 3, 3, 4, 3, 5, 3, 6, 3, 7, 3, 8, 3, 9, 4, 0, 4, 1, 4, 2, 4, 3, 4, 4, 4, 5, 4, 6, 4, 7, 4, 8, 4, 9, 5, 0, 5, 1, 5, 2, 5, 3, 5, 4, 5, 5, 5, 6, 5, 7];
);
