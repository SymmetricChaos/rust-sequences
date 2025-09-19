use num::{BigInt, One, Zero, rational::Ratio};

use crate::core::Factorial;

/// Partial sums of the Taylor series expansion of the sine function evaluated at n/d.
pub struct Sin {
    sum: Ratio<BigInt>,
    val: Ratio<BigInt>,
    x: Ratio<BigInt>,
    factorials: Factorial<BigInt>,
    neg: bool,
}

impl Sin {
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

impl Iterator for Sin {
    type Item = Ratio<BigInt>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.sum.clone();
        self.factorials.next(); // discard one factorial
        let frac = Ratio::new(BigInt::one(), self.factorials.next()?);
        if self.neg {
            self.sum += frac * &self.val;
        } else {
            self.sum -= frac * &self.val;
        }
        self.val *= &self.x;
        self.val *= &self.x;
        self.neg = !self.neg;
        Some(out)
    }
}

/// Partial sums of the Taylor series expansion of the hyperbolic sine function evaluated at n/d.
pub struct SinH {
    sum: Ratio<BigInt>,
    val: Ratio<BigInt>,
    x: Ratio<BigInt>,
    factorials: Factorial<BigInt>,
}

impl SinH {
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
        }
    }
}

impl Iterator for SinH {
    type Item = Ratio<BigInt>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.sum.clone();
        self.factorials.next(); // discard one factorial
        let frac = Ratio::new(BigInt::one(), self.factorials.next()?);
        self.sum += frac * &self.val;
        self.val *= &self.x;
        self.val *= &self.x;
        Some(out)
    }
}

/// Partial sums of the Taylor series expansion of the hyperbolic sine function evaluated at n/d.
pub struct ArcSin {
    sum: Ratio<BigInt>,
    val: Ratio<BigInt>,
    x: Ratio<BigInt>,
    factorials: Factorial<BigInt>,
    factorials_slow: Factorial<BigInt>,
    ctr: u32,
}

impl ArcSin {
    pub fn new<T>(n: T, d: T) -> Self
    where
        BigInt: From<T>,
    {
        let x = Ratio::new(BigInt::from(n), BigInt::from(d)).fract();
        Self {
            sum: Ratio::zero(),
            val: x.clone(),
            x,
            factorials: Factorial::new(),
            factorials_slow: Factorial::new(),
            ctr: 0,
        }
    }
}

impl Iterator for ArcSin {
    type Item = Ratio<BigInt>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.sum.clone();
        let f = self.factorials_slow.next()?;
        let d = BigInt::from(4).pow(self.ctr) * &f * &f * (BigInt::from(2) * self.ctr + 1);
        println!("{}", d);
        let frac = Ratio::new(self.factorials.next()?, d);
        println!("{}", frac);
        self.sum += frac * &self.val;
        self.val *= &self.x;
        self.val *= &self.x;
        self.factorials.next(); // discard one factorial
        self.ctr += 1;
        Some(out)
    }
}

#[cfg(test)]
use crate::core::rational_digits::DecimalDigits;

crate::check_sequences!(
    DecimalDigits::from_ratio(Sin::new(2,1).skip(8).next().unwrap()), 0, 10, [0,9,0,9,2,9,7,4,2,6];
    DecimalDigits::from_ratio(Sin::new(1,3).skip(8).next().unwrap()), 0, 10, [0,3,2,7,1,9,4,6,9,6];
    DecimalDigits::from_ratio(SinH::new(1,1).skip(8).next().unwrap()), 0, 10, [1,1,7,5,2,0,1,1,9,3];
);

crate::print_values!(
    ArcSin::new(1,2), 0, 5;
);
