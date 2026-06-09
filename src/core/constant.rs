use crate::Number;
use num::BigInt;

/// A constant sequence that always returns the same value.
pub struct Constant<T> {
    val: T,
}

impl Constant<Number> {
    pub fn new(val: Number) -> Self {
        Self { val }
    }
}

#[cfg(feature = "big_int")]
impl Constant<BigInt> {
    pub fn new_big<G>(val: G) -> Self
    where
        BigInt: From<G>,
    {
        Self {
            val: BigInt::from(val),
        }
    }
}

impl<T: Clone> Iterator for Constant<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.val.clone())
    }
}

crate::check_sequences!(
    Constant::new_big(3), [3, 3, 3, 3, 3, 3, 3, 3, 3, 3];
    Constant::new(3), [3, 3, 3, 3, 3, 3, 3, 3, 3, 3];
);
