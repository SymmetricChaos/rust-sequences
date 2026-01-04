use crate::core::factorial::Factorials;
use num::{BigInt, One, Zero, rational::Ratio};

/// Partial sums of the Taylor series expansion of the sine function evaluated at n/d.
pub struct Sine {
    sum: Ratio<BigInt>,
    val: Ratio<BigInt>,
    x: Ratio<BigInt>,
    factorials: Factorials<BigInt>,
    neg: bool,
}

impl Sine {
    pub fn new_big<T>(n: T, d: T) -> Self
    where
        BigInt: From<T>,
    {
        let x = Ratio::new(BigInt::from(n), BigInt::from(d));
        Self {
            sum: Ratio::zero(),
            val: x.clone(),
            x,
            factorials: Factorials::new_big(),
            neg: true,
        }
    }
}

impl Iterator for Sine {
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

/// Partial sums of the Taylor series expansion of the cosine function evaluated at n/d.
pub struct Cosine {
    sum: Ratio<BigInt>,
    val: Ratio<BigInt>,
    x: Ratio<BigInt>,
    factorials: Factorials<BigInt>,
    neg: bool,
}

impl Cosine {
    pub fn new_big<T>(n: T, d: T) -> Self
    where
        BigInt: From<T>,
    {
        let x = Ratio::new(BigInt::from(n), BigInt::from(d));
        Self {
            sum: Ratio::zero(),
            val: Ratio::one(),
            x,
            factorials: Factorials::new_big(),
            neg: true,
        }
    }
}

impl Iterator for Cosine {
    type Item = Ratio<BigInt>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.sum.clone();
        let frac = Ratio::new(BigInt::one(), self.factorials.next()?);
        self.factorials.next(); // discard one factorial
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

/// Partial sums of the Taylor series expansion of the cosecant function evaluated at n/d.
pub struct Cosecant {
    sum: Ratio<BigInt>,
    val: Ratio<BigInt>,
    x: Ratio<BigInt>,
    factorials: Factorials<BigInt>,
    neg: bool,
}

impl Cosecant {
    pub fn new_big<T>(n: T, d: T) -> Self
    where
        BigInt: From<T>,
    {
        let x = Ratio::new(BigInt::from(n), BigInt::from(d));
        Self {
            sum: Ratio::zero(),
            val: x.clone(),
            x,
            factorials: Factorials::new_big(),
            neg: true,
        }
    }
}

impl Iterator for Cosecant {
    type Item = Ratio<BigInt>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = if self.sum.is_zero() {
            self.sum.clone()
        } else {
            self.sum.recip().clone()
        };
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

/// Partial sums of the Taylor series expansion of the secant function evaluated at n/d.
pub struct Secant {
    sum: Ratio<BigInt>,
    val: Ratio<BigInt>,
    x: Ratio<BigInt>,
    factorials: Factorials<BigInt>,
    neg: bool,
}

impl Secant {
    pub fn new_big<T>(n: T, d: T) -> Self
    where
        BigInt: From<T>,
    {
        let x = Ratio::new(BigInt::from(n), BigInt::from(d));
        Self {
            sum: Ratio::zero(),
            val: Ratio::one(),
            x,
            factorials: Factorials::new_big(),
            neg: true,
        }
    }
}

impl Iterator for Secant {
    type Item = Ratio<BigInt>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = if self.sum.is_zero() {
            self.sum.clone()
        } else {
            self.sum.recip().clone()
        };
        let frac = Ratio::new(BigInt::one(), self.factorials.next()?);
        self.factorials.next(); // discard one factorial
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

#[cfg(test)]
use crate::core::rational_digits::DecimalDigits;

crate::check_sequences!(
    DecimalDigits::from_ratio_big(Sine::new_big(2,1).skip(10).next().unwrap(), BigInt::from(10)), 0, 10, [0, 9, 0, 9, 2, 9, 7, 4, 2, 6];
    DecimalDigits::from_ratio_big(Sine::new_big(1,3).skip(10).next().unwrap(), BigInt::from(10)), 0, 10, [0, 3, 2, 7, 1, 9, 4, 6, 9, 6];
    DecimalDigits::from_ratio_big(Cosine::new_big(1,3).skip(10).next().unwrap(), BigInt::from(10)), 0, 10, [0, 9, 4, 4, 9, 5, 6, 9, 4, 6];
    DecimalDigits::from_ratio_big(Cosine::new_big(2,1).skip(10).next().unwrap(), BigInt::from(10)), 0, 10, [0, 4, 1, 6, 1, 4, 6, 8, 3, 6];
    DecimalDigits::from_ratio_big(Secant::new_big(1,3).skip(10).next().unwrap(), BigInt::from(10)), 0, 10, [1, 0, 5, 8, 2, 4, 9, 2, 7, 1];
    DecimalDigits::from_ratio_big(Cosecant::new_big(1,3).skip(10).next().unwrap(), BigInt::from(10)), 0, 10, [3, 0, 5, 6, 2, 8, 4, 2, 5, 4];
);
