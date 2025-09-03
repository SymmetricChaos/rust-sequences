use num::BigInt;

/// A constant sequence that always returns the same value.
pub struct Constant {
    val: BigInt,
}

impl Constant {
    pub fn new(val: i64) -> Self {
        Self {
            val: BigInt::from(val),
        }
    }

    pub fn new_big(val: BigInt) -> Self {
        Self { val }
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
