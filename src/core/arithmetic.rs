use num::{BigInt, CheckedAdd, Zero};

use crate::Number;

/// Arithmetic sequence with chosen initial value and increment
pub struct Arithmetic<T> {
    val: T,
    inc: T,
}

impl Arithmetic<Number> {
    pub fn new(init: Number, inc: Number) -> Self {
        Self { val: init, inc }
    }
}

#[cfg(feature = "big_int")]
impl Arithmetic<BigInt> {
    pub fn new_big<G>(init: G, inc: G) -> Self
    where
        BigInt: From<G>,
    {
        Self {
            val: BigInt::from(init),
            inc: BigInt::from(inc),
        }
    }
}

impl<T: Clone + CheckedAdd> Iterator for Arithmetic<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.val.checked_add(&self.inc)?;
        Some(out)
    }
}

/// The non-negative multiples of a value. The simplest arithmetic sequence.
pub struct Multiples<T> {
    val: T,
    inc: T,
}

impl Multiples<Number> {
    pub fn new(init: Number) -> Self {
        Self { val: 0, inc: init }
    }
}

#[cfg(feature = "big_int")]
impl Multiples<BigInt> {
    pub fn new_big<G>(init: G) -> Self
    where
        BigInt: From<G>,
    {
        Self {
            val: BigInt::zero(),
            inc: BigInt::from(init),
        }
    }
}

impl<T: Clone + CheckedAdd> Iterator for Multiples<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.val.checked_add(&self.inc)?;
        Some(out)
    }
}

crate::check_iteration_times!(
    Arithmetic::new_big(4, 3), 3_500_000;
    Arithmetic::new(4, 3), 3_500_000;
    Arithmetic::new(4, 3), 3_500_000;
);

crate::check_sequences!(
    Arithmetic::new_big(0, 0), [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    Arithmetic::new_big(1, 1), [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    Arithmetic::new_big(3, 2), [3, 5, 7, 9, 11, 13, 15, 17, 19, 21];
    Arithmetic::new_big(4, 3), [4, 7, 10, 13, 16, 19, 22, 25, 28, 31];
);
