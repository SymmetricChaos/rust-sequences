use crate::Number;
use num::{BigInt, CheckedAdd, One, Zero};

/// The natural numbers. The non-negative integers.
///
/// ```text
/// 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18...
/// ```
pub struct Naturals<T> {
    ctr: T,
}

impl Naturals<Number> {
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

#[cfg(feature = "big_int")]
impl Naturals<BigInt> {
    pub fn new_big() -> Self {
        Self {
            ctr: BigInt::zero(),
        }
    }
}

impl<T: CheckedAdd + Clone + One> Iterator for Naturals<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.ctr.clone();
        self.ctr = self.ctr.checked_add(&T::one())?;
        Some(out)
    }
}

crate::check_sequences!(
    Naturals::new_big(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    Naturals::new(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
);

crate::sample_sequences!(
    Naturals::new();
);
