use crate::Number;
use num::{BigInt, CheckedMul, CheckedSub, FromPrimitive, Integer, One, Zero};

/// Any recurrence of the form
/// ```text
/// a_x = p * a_{x-1} - q * a_{x-2}
/// ```
/// beginning with a_0 = 0 and a_1 = 1
pub struct LucasU<T> {
    a: T,
    b: T,
    p: T,
    q: T,
}

impl LucasU<Number> {
    pub fn new(p: Number, q: Number) -> Self {
        Self { a: 0, b: 1, p, q }
    }
}

#[cfg(feature = "big_int")]
impl LucasU<BigInt> {
    pub fn new_big<G>(p: G, q: G) -> Self
    where
        BigInt: From<G>,
    {
        Self {
            a: BigInt::zero(),
            b: BigInt::one(),
            p: BigInt::from(p),
            q: BigInt::from(q),
        }
    }
}

impl<T: Clone + CheckedSub + CheckedMul + Integer> Iterator for LucasU<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let x = self.p.checked_mul(&self.a)?;
        let y = self.q.checked_mul(&self.b)?;
        let t = x.checked_sub(&y)?;
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

/// Any recurrence of the form
/// ```text
/// a_x = p * a_{x-1} - q * a_{x-2}
/// ```
/// beginning with a_0 = 2 and a_1 = p
pub struct LucasV<T> {
    a: T,
    b: T,
    p: T,
    q: T,
}

impl LucasV<Number> {
    pub fn new(p: Number, q: Number) -> Self {
        Self { a: 2, b: p, p, q }
    }
}

#[cfg(feature = "big_int")]
impl LucasV<BigInt> {
    pub fn new_big<G: Clone>(p: G, q: G) -> Self
    where
        BigInt: From<G>,
    {
        Self {
            a: BigInt::from_i32(2).unwrap(),
            b: BigInt::from(p.clone()),
            p: BigInt::from(p),
            q: BigInt::from(q),
        }
    }
}

impl<T: Clone + CheckedSub + CheckedMul + Integer> Iterator for LucasV<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let x = self.p.checked_mul(&self.a)?;
        let y = self.q.checked_mul(&self.b)?;
        let t = x.checked_sub(&y)?;
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

crate::check_sequences!(
    LucasU::new(1, -1), [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]; // Fibonacci
    LucasU::new_big(1, -1), [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]; // Fibonacci
    LucasV::new(1, -1), [2, 1, 3, 4, 7, 11, 18, 29, 47, 76]; // Lucas
    LucasU::new(1, -2), [0, 1, 2, 5, 12, 29, 70, 169, 408, 985]; // Pell
);
