use num::{BigInt, CheckedMul, CheckedSub, One, Zero};

/// Any recurrence of the form
/// a_x = p * a_{x-1} - q * a_{x-2}
/// beginning with a_0 = 0 and a_1 = 1
pub struct LucasU<T> {
    a: T,
    b: T,
    p: T,
    q: T,
}

impl<T: CheckedMul + CheckedSub + Clone + One + Zero> LucasU<T> {
    pub fn new(p: T, q: T) -> Self {
        Self {
            a: T::zero(),
            b: T::one(),
            p,
            q,
        }
    }
}

impl LucasU<BigInt> {
    pub fn new_big<T>(p: T, q: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            a: BigInt::zero(),
            b: BigInt::one(),
            p: BigInt::from(p),
            q: BigInt::from(q),
        }
    }
}

impl<T: CheckedMul + CheckedSub + Clone> Iterator for LucasU<T> {
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
/// a_x = p * a_{x-1} - q * a_{x-2}
/// beginning with a_0 = 2 and a_1 = p
pub struct LucasV<T> {
    a: T,
    b: T,
    p: T,
    q: T,
}

impl<T: CheckedMul + CheckedSub + Clone + One + Zero> LucasV<T> {
    pub fn new(p: T, q: T) -> Self {
        Self {
            a: T::one() + T::one(),
            b: p.clone(),
            p,
            q,
        }
    }
}

impl LucasV<BigInt> {
    pub fn new_big<N>(p: N, q: N) -> Self
    where
        BigInt: From<N>,
    {
        let p = BigInt::from(p);
        Self {
            a: BigInt::one() + BigInt::one(),
            b: p.clone(),
            p: p,
            q: BigInt::from(q),
        }
    }
}

impl<T: CheckedMul + CheckedSub + Clone> Iterator for LucasV<T> {
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
    LucasU::new_big(1, -1), 0, 10, [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]; // Fibonacci
    LucasV::new_big(1, -1), 0, 10, [2, 1, 3, 4, 7, 11, 18, 29, 47, 76]; // Lucas
    LucasU::new_big(1, -2), 0, 10, [0, 1, 2, 5, 12, 29, 70, 169, 408, 985]; // Pell
);
