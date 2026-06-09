use crate::Number;
use num::BigInt;

/// A constant sequence that always returns the same value.
///
/// ```text
/// n = 3
/// 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3...
/// ```
pub struct Constant<T> {
    n: T,
}

impl Constant<Number> {
    pub fn new(n: Number) -> Self {
        Self { n }
    }
}

#[cfg(feature = "big_int")]
impl Constant<BigInt> {
    pub fn new_big<G>(n: G) -> Self
    where
        BigInt: From<G>,
    {
        Self { n: BigInt::from(n) }
    }
}

impl<T: Clone> Iterator for Constant<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.n.clone())
    }
}

crate::check_sequences!(
    Constant::new_big(3), [3, 3, 3, 3, 3, 3, 3, 3, 3, 3];
    Constant::new(3), [3, 3, 3, 3, 3, 3, 3, 3, 3, 3];
);

crate::sample_sequences!(
    Constant::new(3);
);
