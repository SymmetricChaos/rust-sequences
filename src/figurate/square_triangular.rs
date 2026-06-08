use crate::Number;
use num::{BigInt, One, Zero};

/// The square triangular numbers.
///
/// ```text
/// 0, 1, 36, 1225, 41616, 1413721, 48024900...
/// ```
pub struct SquareTriangular<T> {
    a: T,
    b: T,
}

impl SquareTriangular<Number> {
    pub fn new() -> Self {
        Self { a: 0, b: 1 }
    }
}

#[cfg(feature = "big_int")]
impl SquareTriangular<BigInt> {
    pub fn new_big() -> Self {
        Self {
            a: BigInt::zero(),
            b: BigInt::one(),
        }
    }
}

impl Iterator for SquareTriangular<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a;
        let t = (self.b.checked_mul(34)? - self.a).checked_add(2)?;
        self.a = self.b;
        self.b = t;
        Some(out)
    }
}

#[cfg(feature = "big_int")]
impl Iterator for SquareTriangular<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let t = (&self.b * 34 - &self.a) + 2;
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

crate::check_sequences!(
    SquareTriangular::new_big(), [0_u128, 1, 36, 1225, 41616, 1413721, 48024900, 1631432881, 55420693056, 1882672131025, 63955431761796, 2172602007770041, 73804512832419600, 2507180834294496361, 85170343853180456676, 2893284510173841030625, 98286503002057414584576, 3338847817559778254844961, 113422539294030403250144100];
);
