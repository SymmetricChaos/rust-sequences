use crate::Number;
use num::{BigInt, Integer};

/// The values of a generalized Collatz sequence with with constants a and b.
///
/// ```text
/// f(n) = an+b for odd n
///      = n/2  for even n
/// ```
pub struct CollatzGeneral<T> {
    n: T,
    a: T,
    b: T,
}

impl CollatzGeneral<Number> {
    pub fn new(n: Number, a: Number, b: Number) -> Self {
        Self { n, a, b }
    }
}

impl CollatzGeneral<BigInt> {
    pub fn new_big<T>(n: T, a: T, b: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            n: BigInt::from(n),
            a: BigInt::from(a),
            b: BigInt::from(b),
        }
    }
}

impl Iterator for CollatzGeneral<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.n;
        if out.is_even() {
            self.n /= 2;
        } else {
            self.n = self.n.checked_mul(self.a)?;
            self.n = self.n.checked_add(self.b)?;
        }
        Some(out)
    }
}

impl Iterator for CollatzGeneral<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.n.clone();
        if out.is_even() {
            self.n /= 2;
        } else {
            self.n *= &self.a;
            self.n += &self.b;
        }
        Some(out)
    }
}
