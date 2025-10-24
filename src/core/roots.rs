use num::{BigInt, CheckedAdd, CheckedDiv, CheckedMul, Integer, One, rational::Ratio};

/// Convergents of the principal square root of a rational number by Newton's Method.
pub struct SquareRoot<T> {
    convergent: Ratio<T>,
    s: Ratio<T>,
}

impl<T: CheckedAdd + CheckedDiv + CheckedMul + Clone + Integer + One> SquareRoot<T> {
    pub fn new(numer: T, denom: T) -> Self {
        let n = numer;
        let d = denom;
        Self {
            convergent: Ratio::new(n.clone(), d.clone()),
            s: Ratio::new(n.clone(), d.clone()),
        }
    }
}

impl SquareRoot<BigInt> {
    pub fn new_big<N>(numer: N, denom: N) -> Self
    where
        BigInt: From<N>,
    {
        let n = BigInt::from(numer);
        let d = BigInt::from(denom);
        Self {
            convergent: Ratio::new(n.clone(), d.clone()),
            s: Ratio::new(n.clone(), d.clone()),
        }
    }
}

impl<T: CheckedAdd + CheckedDiv + CheckedMul + Clone + Integer + One> Iterator for SquareRoot<T> {
    type Item = Ratio<T>;

    // (x+s/x)/2
    fn next(&mut self) -> Option<Self::Item> {
        let out = self.convergent.clone();
        let half = Ratio::new(T::one(), T::one() + T::one());
        self.convergent = self
            .convergent
            .checked_add(&self.s.checked_div(&self.convergent)?)?
            .checked_mul(&half)?;
        Some(out)
    }
}

/// Convergents of the principal cube root of a rational number by Newton's Method.
pub struct CubeRoot<T> {
    convergent: Ratio<T>,
    s: Ratio<T>,
}

impl<T: CheckedAdd + CheckedDiv + CheckedMul + Clone + Integer + One> CubeRoot<T> {
    pub fn new(num: T, den: T) -> Self {
        let n = num;
        let d = den;
        Self {
            convergent: Ratio::new(n.clone(), d.clone()),
            s: Ratio::new(n.clone(), d.clone()),
        }
    }
}

impl CubeRoot<BigInt> {
    pub fn new_big<N>(num: N, den: N) -> Self
    where
        BigInt: From<N>,
    {
        let n = BigInt::from(num);
        let d = BigInt::from(den);
        Self {
            convergent: Ratio::new(n.clone(), d.clone()),
            s: Ratio::new(n.clone(), d.clone()),
        }
    }
}

impl<T: CheckedAdd + CheckedDiv + CheckedMul + Clone + Integer + One> Iterator for CubeRoot<T> {
    type Item = Ratio<T>;

    // (s/x^2 + 2x)/3
    fn next(&mut self) -> Option<Self::Item> {
        let out = self.convergent.clone();
        let two_x = self.convergent.checked_add(&self.convergent)?;
        let third = Ratio::new(T::one(), T::one() + T::one() + T::one());
        let squ = self.convergent.checked_mul(&self.convergent)?;
        self.convergent = self
            .s
            .checked_div(&squ)?
            .checked_add(&two_x)?
            .checked_mul(&third)?;
        Some(out)
    }
}
