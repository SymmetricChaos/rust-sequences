use num::{BigInt, CheckedAdd, CheckedMul, CheckedSub, One, PrimInt};

/// All primitive Pythagorean triples
pub struct PythagoreanTriples<T> {
    m: T,
    n: T,
}

impl<T: PrimInt> PythagoreanTriples<T> {
    pub fn new() -> Self {
        Self {
            m: T::one() + T::one(),
            n: T::one(),
        }
    }
}

impl PythagoreanTriples<BigInt> {
    pub fn new_big() -> Self {
        Self {
            m: BigInt::one() + BigInt::one(),
            n: BigInt::one(),
        }
    }
}

impl<T: CheckedAdd + CheckedMul + CheckedSub + One> Iterator for PythagoreanTriples<T> {
    type Item = (T, T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let m2 = self.m.checked_mul(&self.m)?;
        let n2 = self.n.checked_mul(&self.n)?;
        let a = m2.checked_add(&n2)?;
        let b = self
            .m
            .checked_mul(&self.n)?
            .checked_mul(&(T::one() + T::one()))?;
        let c = m2.checked_sub(&n2)?;
        Some((a, b, c))
    }
}
