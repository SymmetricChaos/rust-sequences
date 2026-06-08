use crate::{Number, factorial::Factorial};
use num::{BigInt, CheckedAdd, CheckedMul, One, Zero, rational::Ratio};

/// The partial sums of the Taylor series form of the exponential function evaluated at numer/denom.
pub struct Exponential<T> {
    sum: Ratio<T>,
    val: Ratio<T>,
    x: Ratio<T>,
    factorials: Factorial<T>,
}

impl Exponential<Number> {
    pub fn new(numer: Number, denom: Number) -> Self {
        Self {
            sum: Ratio::zero(),
            val: Ratio::one(),
            x: Ratio::new(numer, denom),
            factorials: Factorial::new(),
        }
    }

    pub fn from_ratio(x: Ratio<Number>) -> Self {
        Self {
            sum: Ratio::one(),
            val: Ratio::one(),
            x,
            factorials: Factorial::new(),
        }
    }
}

#[cfg(feature = "big_int")]
impl Exponential<BigInt> {
    pub fn new_big<G>(numer: G, denom: G) -> Self
    where
        BigInt: From<G>,
    {
        Self {
            sum: Ratio::zero(),
            val: Ratio::one(),
            x: Ratio::new(BigInt::from(numer), BigInt::from(denom)),
            factorials: Factorial::new_big(),
        }
    }

    pub fn from_ratio_big<G: Clone>(x: Ratio<G>) -> Self
    where
        BigInt: From<G>,
    {
        let x = Ratio::new(
            BigInt::from(x.numer().clone()),
            BigInt::from(x.denom().clone()),
        );
        Self {
            sum: Ratio::zero(),
            val: Ratio::one(),
            x,
            factorials: Factorial::new_big(),
        }
    }
}

impl Iterator for Exponential<Number> {
    type Item = Ratio<Number>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.sum.clone();
        self.val = self.val.checked_mul(&self.x)?;
        let frac = Ratio::new(1, self.factorials.next()?);
        self.sum = self.sum.checked_add(&frac.checked_mul(&self.val)?)?;
        Some(out)
    }
}

#[cfg(feature = "big_int")]
impl Iterator for Exponential<BigInt> {
    type Item = Ratio<BigInt>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.sum.clone();
        self.val = self.val.checked_mul(&self.x)?;
        let frac = Ratio::new(BigInt::one(), self.factorials.next()?);
        self.sum = self.sum.checked_add(&frac.checked_mul(&self.val)?)?;
        Some(out)
    }
}

/// The partial sums of the of the natural logarithm evaluted at a rational number.
/// Calculated using the inverse hyperbolic tangent formula.
pub struct NaturalLog<T> {
    sum: Ratio<T>,
    val: Ratio<T>,
    x: Ratio<T>,
    ctr: T,
}

impl NaturalLog<Number> {
    /// Panics if numer/denom <= 0.
    pub fn new(numer: Number, denom: Number) -> Self {
        assert!(Ratio::new(numer, denom) > Ratio::zero());
        let x_minus_one = Ratio::new(numer, denom) - Ratio::one();
        let x_plus_one = Ratio::new(numer, denom) + Ratio::one();
        Self {
            sum: Ratio::zero(),
            val: x_minus_one / x_plus_one,
            x: x_minus_one / x_plus_one,
            ctr: 1,
        }
    }

    /// Panics if x <= 0.
    pub fn from_ratio(x: Ratio<Number>) -> Self {
        assert!(x > Ratio::zero());
        let x_minus_one = x - Ratio::one();
        let x_plus_one = x + Ratio::one();
        Self {
            sum: Ratio::zero(),
            val: x_minus_one / x_plus_one,
            x: x_minus_one / x_plus_one,
            ctr: 1,
        }
    }
}

#[cfg(feature = "big_int")]
impl NaturalLog<BigInt> {
    /// Panics if numer/denom <= 0.
    pub fn new_big<G>(numer: G, denom: G) -> Self
    where
        BigInt: From<G>,
    {
        let x = Ratio::new(BigInt::from(numer), BigInt::from(denom));
        assert!(x > Ratio::zero());
        Self {
            sum: Ratio::zero(),
            val: (x.clone() - Ratio::one()) / (x.clone() + Ratio::one()),
            x: (x.clone() - Ratio::one()) / (x + Ratio::one()),
            ctr: BigInt::one(),
        }
    }

    /// Panics if x <= 0.
    pub fn from_ratio_big<G: Clone>(x: Ratio<G>) -> Self
    where
        BigInt: From<G>,
    {
        let x = Ratio::new(
            BigInt::from(x.numer().clone()),
            BigInt::from(x.denom().clone()),
        );
        assert!(x > Ratio::zero());
        Self {
            sum: Ratio::zero(),
            val: (x.clone() - Ratio::one()) / (x.clone() + Ratio::one()),
            x: (x.clone() - Ratio::one()) / (x + Ratio::one()),
            ctr: BigInt::one(),
        }
    }
}

impl Iterator for NaturalLog<Number> {
    type Item = Ratio<Number>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.sum.clone();

        let frac = Ratio::new(1, self.ctr.clone());

        self.sum = self.sum.checked_add(&frac.checked_mul(&self.val)?)?;

        // Multiply twice to get an odd power
        self.val = self.val.checked_mul(&self.x)?;
        self.val = self.val.checked_mul(&self.x)?;

        self.ctr = self.ctr.checked_add(2)?;

        // Times two
        Some(out + out)
    }
}

#[cfg(feature = "big_int")]
impl Iterator for NaturalLog<BigInt> {
    type Item = Ratio<BigInt>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.sum.clone();

        let frac = Ratio::new(BigInt::one(), self.ctr.clone());

        self.sum = self.sum.checked_add(&frac.checked_mul(&self.val)?)?;

        // Multiply twice to get an odd power
        self.val = &self.val * &self.x * &self.x;

        // Add one twice to get an odd integer
        self.ctr += BigInt::from(2);

        // Times two
        Some(&out + &out)
    }
}

crate::print_sequences!(
    Exponential::new_big(1,1), 15; // converges on e
    Exponential::new(1,1), 15; // converges on e
    NaturalLog::new(3,2), 10; // note that overflow end the sequence
    NaturalLog::from_ratio_big(Ratio::new(3,2)), 5;
    NaturalLog::new_big(8,1), 5;
);
