use num::{BigInt, One, Zero, rational::Ratio};

use crate::core::Factorial;

/// Partial sums of the Taylor series expansion of the sine function evaluated at x
pub struct Sine {
    sum: Ratio<BigInt>,
    val: Ratio<BigInt>,
    x: Ratio<BigInt>,
    factorials: Factorial<BigInt>,
    neg: bool,
}

impl Sine {
    pub fn new<T>(n: T, d: T) -> Self
    where
        BigInt: From<T>,
    {
        let x = Ratio::new(BigInt::from(n), BigInt::from(d));
        Self {
            sum: Ratio::zero(),
            val: x.clone(),
            x,
            factorials: Factorial::new(),
            neg: true,
        }
    }
}

impl Iterator for Sine {
    type Item = Ratio<BigInt>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.sum.clone();
        let frac = Ratio::new(BigInt::one(), self.factorials.next()?);
        if self.neg {
            self.sum += frac * &self.val;
        } else {
            self.sum -= frac * &self.val;
        }
        self.val *= &self.x;
        self.val *= &self.x;
        self.factorials.next(); // discard one factorial
        self.neg = !self.neg;
        Some(out)
    }
}

crate::print_values!(
    Sine::new(2,1), 0, 7; // converges to 0.90929742682...
    Sine::new(1,3), 0, 7; // converges to 0.32719469679...
);
