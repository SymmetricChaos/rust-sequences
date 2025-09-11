use crate::core::Factorial;
use num::{BigInt, One, rational::Ratio};

/// The partial sums of the Taylor series form of the exponential function evaluated at x.
pub struct Exponential {
    sum: Ratio<BigInt>,
    val: Ratio<BigInt>,
    x: Ratio<BigInt>,
    factorials: Factorial,
}

impl Exponential {
    pub fn new<T>(n: T, d: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            sum: Ratio::one(),
            val: Ratio::one(),
            x: Ratio::new(BigInt::from(n), BigInt::from(d)),
            factorials: Factorial::new(),
        }
    }
}

impl Iterator for Exponential {
    type Item = Ratio<BigInt>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.sum.clone();
        self.val *= &self.x;
        let frac = Ratio::new(BigInt::one(), self.factorials.next()?);
        self.sum += frac * &self.val;
        Some(out)
    }
}

crate::print_values!(
    Exponential::new(1,1), 0, 10; // converges on e
);
