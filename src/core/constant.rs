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

crate::check_sequences!(
    Constant::new(3), 0, 10, [3, 3, 3, 3, 3, 3, 3, 3, 3, 3];
);
