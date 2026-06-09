use num::{BigInt, CheckedAdd, Zero};

use crate::Number;

/// Arithmetic sequence with chosen initial value and increment.
///
/// ```text
/// init = 3, incr = 2
/// 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37...
///
/// init = -52, incr = 19
/// -52, -33, -14, 5, 24, 43, 62, 81, 100, 119, 138, 157, 176, 195, 214...
/// ```
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

/// The multiples of a value.
///
/// ```text
/// init = 41
/// 0, 41, 82, 123, 164, 205, 246, 287, 328, 369, 410, 451, 492, 533...
///
/// init = -7
/// 0, -7, -14, -21, -28, -35, -42, -49, -56, -63, -70, -77, -84, -91...
/// ```
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

crate::sample_sequences!(
    Arithmetic::new(-52,19);
    Arithmetic::new(3,2);
    Multiples::new(-7);
    Multiples::new(41);
);
