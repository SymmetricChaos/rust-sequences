use num::BigInt;

/// A constant sequence that always returns the same value.
pub struct Constant {
    val: BigInt,
}

impl Constant {
    pub fn new<T>(val: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            val: BigInt::from(val),
        }
    }
}

impl Iterator for Constant {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.val.clone())
    }
}

/// A constant sequence that always returns the same value.
pub struct ConstantGeneric<T> {
    val: T,
}

impl<T> ConstantGeneric<T> {
    pub fn news(val: T) -> Self {
        Self { val }
    }
}

impl<T: Clone> Iterator for ConstantGeneric<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.val.clone())
    }
}

crate::check_sequences!(
    Constant::new(3), 0, 10, [3, 3, 3, 3, 3, 3, 3, 3, 3, 3];
);
