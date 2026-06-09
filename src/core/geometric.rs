use crate::Number;
use num::{BigInt, CheckedMul};

/// Geometric sequence with chosen initial value and multiplier
///
/// ```text
/// initial = 3, multiplier = 2
/// 3, 6, 12, 24, 48, 96, 192, 384, 768, 1536, 3072, 6144, 12288, 24576...
///
/// initial = 8, multiplier = -5
/// 8, -40, 200, -1000, 5000, -25000, 125000, -625000, 3125000...
/// ```
pub struct Geometric<T> {
    value: T,
    multiplier: T,
}

impl Geometric<Number> {
    pub fn new(initial: Number, multiplier: Number) -> Self {
        Self {
            value: initial,
            multiplier,
        }
    }
}

#[cfg(feature = "big_int")]
impl Geometric<BigInt> {
    pub fn new_big<G>(initial: G, multiplier: G) -> Self
    where
        BigInt: From<G>,
    {
        Self {
            value: BigInt::from(initial),
            multiplier: BigInt::from(multiplier),
        }
    }
}

impl<T: CheckedMul + Clone> Iterator for Geometric<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.value.clone();
        self.value = self.value.checked_mul(&self.multiplier)?;
        Some(out)
    }
}

crate::check_sequences!(
    Geometric::new(3, 2), [3, 6, 12, 24, 48, 96, 192, 384, 768, 1536];
    Geometric::new(4, 3), [4, 12, 36, 108, 324, 972, 2916, 8748, 26244, 78732];
);

crate::sample_sequences!(
   Geometric::new(3, 2);
   Geometric::new(8, -5);
);
