use num::{BigInt, PrimInt};

/// Geometric sequence with chosen initial value and multiplier
pub struct Geometric {
    val: BigInt,
    mul: BigInt,
}

impl Geometric {
    pub fn new<T>(init: T, mul: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            val: BigInt::from(init),
            mul: BigInt::from(mul),
        }
    }
}

impl Iterator for Geometric {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val *= &self.mul;
        Some(out)
    }
}

/// Geometric sequence with chosen initial value and multiplier
pub struct GeometricGeneric<T> {
    val: T,
    mul: T,
}

impl<T: PrimInt> GeometricGeneric<T> {
    pub fn new(init: T, mul: T) -> Self {
        Self { val: init, mul }
    }
}

impl<T: PrimInt> Iterator for GeometricGeneric<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.val.checked_mul(&self.mul)?;
        Some(out)
    }
}

crate::print_values!(
    GeometricGeneric::<i32>::new(3, -4), 0, 10;
);

crate::check_sequences!(
    Geometric::new(3, 2), 0, 10, [3, 6, 12, 24, 48, 96, 192, 384, 768, 1536];
    Geometric::new(4, 3), 0, 10, [4, 12, 36, 108, 324, 972, 2916, 8748, 26244, 78732];
);
