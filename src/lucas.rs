use crate::Number;
use num::{BigInt, CheckedAdd, Integer, One};

/// The Lucas numbers. Defined by the same recurrence as the Fibonacci numbers but starting with 2, 1 rather than 1, 1.
///
/// ```text
/// 2, 1, 3, 4, 7, 11, 18, 29, 47, 76, 123, 199, 322, 521, 843, 1364...
/// ```
pub struct Lucas<T> {
    a: T,
    b: T,
}

impl Lucas<Number> {
    pub fn new() -> Self {
        Self { a: 2, b: 1 }
    }
}

#[cfg(feature = "big_int")]
impl Lucas<BigInt> {
    pub fn new_big() -> Self {
        Self {
            a: BigInt::from(2),
            b: BigInt::one(),
        }
    }
}

impl<T: Clone + CheckedAdd + Integer> Iterator for Lucas<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let t = self.a.checked_add(&self.b)?;
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

crate::check_iteration_times!(
    Lucas::new_big(), 157_500;
);

crate::check_sequences!(
    Lucas::new_big(), [2, 1, 3, 4, 7, 11, 18, 29, 47, 76, 123, 199, 322, 521, 843, 1364, 2207, 3571, 5778, 9349, 15127, 24476, 39603, 64079, 103682, 167761, 271443, 439204, 710647, 1149851, 1860498, 3010349, 4870847, 7881196, 12752043, 20633239, 33385282, 54018521, 87403803];
);

crate::sample_sequences!(
    Lucas::new();
);
