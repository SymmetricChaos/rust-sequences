use num::{BigInt, One, Zero, rational::Ratio};

use crate::core::Factorial;

/// Partial sums of the Taylor series expansion of the sine function evaluated at x.
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

/// Partial sums of the Taylor series expansion of the hyperbolic sine function evaluated at x.
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
        let frac = Ratio::new(BigInt::one(), self.factorials.next()?);
        self.sum += frac * &self.val;
        self.val *= &self.x;
        self.val *= &self.x;
        self.factorials.next(); // discard one factorial
        Some(out)
    }
}

// /// Partial sums of the Taylor series expansion of the hyperbolic sine function evaluated at x.
// pub struct ArcSin {
//     sum: Ratio<BigInt>,
//     val: Ratio<BigInt>,
//     x: Ratio<BigInt>,
//     factorials: Factorial<BigInt>,
//     factorials_slow: Factorial<BigInt>,
//     ctr: u32,
// }

// impl ArcSin {
//     pub fn new<T>(n: T, d: T) -> Self
//     where
//         BigInt: From<T>,
//     {
//         let x = Ratio::new(BigInt::from(n), BigInt::from(d)).fract();
//         Self {
//             sum: Ratio::zero(),
//             val: x.clone(),
//             x,
//             factorials: Factorial::new(),
//             factorials_slow: Factorial::new(),
//             ctr: 0,
//         }
//     }
// }

// impl Iterator for ArcSin {
//     type Item = Ratio<BigInt>;

//     fn next(&mut self) -> Option<Self::Item> {
//         let out = self.sum.clone();
//         let f = self.factorials_slow.next()?;
//         let d = BigInt::from(4).pow(self.ctr) * &f * &f * (BigInt::from(2) * self.ctr + 1);
//         let frac = Ratio::new(self.factorials.next()?, d);
//         println!("{}", frac);
//         self.sum += frac * &self.val;
//         self.val *= &self.x;
//         self.val *= &self.x;
//         self.factorials.next(); // discard one factorial
//         self.ctr += 1;
//         Some(out)
//     }
// }

crate::print_values!(
    Sin::new(2,1), 0, 7; // converges to 0.90929742682...
    Sin::new(1,3), 0, 6; // converges to 0.32719469679...
    SinH::new(1,1), 0, 7; // converges to 1.17520119364...
);
