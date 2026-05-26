use num::{BigInt, CheckedAdd, CheckedDiv, CheckedMul, Integer};

/// The values of a generalized Collatz sequence.
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

impl<T: Clone + CheckedAdd + CheckedMul + CheckedDiv + Integer> CollatzGeneral<T> {
    pub fn new(n: T, a: T, b: T) -> Self {
        Self { n, a, b }
    }
}

impl CollatzGeneral<BigInt> {
    pub fn new_big<T>(n: T, a: T, b: T) -> Self
    where
        BigInt: From<T>,
    {
        Self::new(BigInt::from(n), BigInt::from(a), BigInt::from(b))
    }
}

impl<T: Clone + CheckedAdd + CheckedMul + CheckedDiv + Integer> Iterator for CollatzGeneral<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.n.clone();
        if out.is_even() {
            self.n = self.n.checked_div(&(T::one() + T::one()))?;
        } else {
            self.n = self.n.checked_mul(&self.a)?;
            self.n = self.n.checked_add(&self.b)?;
        }
        Some(out)
    }
}
