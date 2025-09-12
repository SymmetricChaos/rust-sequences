use num::{BigInt, rational::Ratio};

/// Convergents of the square root of a rational number by Newton's Method (Heron's Method).
pub struct SquareRoot {
    convergent: Ratio<BigInt>,
    s: Ratio<BigInt>,
}

impl SquareRoot {
    pub fn new<T>(num: T, den: T) -> Self
    where
        BigInt: From<T>,
    {
        let n = BigInt::from(num);
        let d = BigInt::from(den);
        Self {
            convergent: Ratio::new(n.clone(), d.clone()),
            s: Ratio::new(n.clone(), d.clone()),
        }
    }
}

impl Iterator for SquareRoot {
    type Item = Ratio<BigInt>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.convergent.clone();
        self.convergent = (&self.convergent + &self.s / &self.convergent) / BigInt::from(2);
        Some(out)
    }
}

/// Convergents of the cube root of a rational number by Newton's Method (Heron's Method).
pub struct CubeRoot {
    convergent: Ratio<BigInt>,
    s: Ratio<BigInt>,
}

impl CubeRoot {
    pub fn new<T>(num: T, den: T) -> Self
    where
        BigInt: From<T>,
    {
        let n = BigInt::from(num);
        let d = BigInt::from(den);
        Self {
            convergent: Ratio::new(n.clone(), d.clone()),
            s: Ratio::new(n.clone(), d.clone()),
        }
    }
}

impl Iterator for CubeRoot {
    type Item = Ratio<BigInt>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.convergent.clone();
        self.convergent = ((&self.convergent * BigInt::from(2))
            + &self.s / (&self.convergent * &self.convergent))
            / BigInt::from(3);
        Some(out)
    }
}
