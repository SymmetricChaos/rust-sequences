use crate::{Number, core::traits::Increment};
use num::{BigInt, One, Zero};

/// The number of derangements for a set of n elements (starting from 0).
///
/// ```text
/// 1, 0, 1, 2, 9, 44, 265, 1854...
/// ```
pub struct Derangements<T> {
    a: T,
    b: T,
    ctr: T,
    overflowed: bool,
}

impl Derangements<Number> {
    pub fn new() -> Self {
        Self {
            a: 1,
            b: 0,
            ctr: 1,
            overflowed: false,
        }
    }
}

impl Derangements<BigInt> {
    pub fn new_big() -> Self {
        Self {
            a: BigInt::one(),
            b: BigInt::zero(),
            ctr: BigInt::one(),
            overflowed: false,
        }
    }
}

impl Iterator for Derangements<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        if self.overflowed {
            return None;
        }

        let out = self.a;

        let next = match self.ctr.checked_mul(self.a.checked_add(self.b)?) {
            Some(n) => n,
            None => {
                self.overflowed = true;
                return Some(out);
            }
        };
        self.a = self.b;
        self.b = next;
        self.ctr.incr()?;

        Some(out)
    }
}

impl Iterator for Derangements<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();

        let next = &self.ctr * (&self.a + &self.b);
        self.a = self.b.clone();
        self.b = next;
        self.ctr.incr()?;

        Some(out)
    }
}

crate::check_sequences!(
    Derangements::new(), [1_i64, 0, 1, 2, 9, 44, 265, 1854, 14833, 133496, 1334961, 14684570, 176214841, 2290792932, 32071101049, 481066515734, 7697064251745, 130850092279664, 2355301661033953, 44750731559645106];
);
