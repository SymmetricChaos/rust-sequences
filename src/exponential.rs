use crate::{Number, core::traits::Increment, factorial::Factorial};
use num::{BigInt, CheckedAdd, CheckedMul, Integer, One, Zero, rational::Ratio};

/// The partial sums of the Taylor series form of the exponential function evaluated at a rational number.
///
/// ```text
/// q = 1/1
/// 0, 1, 2, 5/2, 8/3, 65/24, 163/60, 1957/720, 685/252, 109601/40320...
///
/// q = 3/2
/// 0, 3/2, 15/4, 87/16, 201/32, 1689/256, 17133/2560, 13755/2048...
/// ```
pub struct Exponential<T> {
    sum: Ratio<T>,
    val: Ratio<T>,
    q: Ratio<T>,
    factorials: Factorial<T>,
}

impl Exponential<Number> {
    pub fn new(numer: Number, denom: Number) -> Self {
        Self {
            sum: Ratio::zero(),
            val: Ratio::one(),
            q: Ratio::new(numer, denom),
            factorials: Factorial::new(),
        }
    }

    pub fn from_ratio(q: Ratio<Number>) -> Self {
        Self {
            sum: Ratio::one(),
            val: Ratio::one(),
            q,
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
            q: Ratio::new(BigInt::from(numer), BigInt::from(denom)),
            factorials: Factorial::new_big(),
        }
    }

    pub fn from_ratio_big<G: Clone>(q: Ratio<G>) -> Self
    where
        BigInt: From<G>,
    {
        let q = Ratio::new(
            BigInt::from(q.numer().clone()),
            BigInt::from(q.denom().clone()),
        );
        Self {
            sum: Ratio::zero(),
            val: Ratio::one(),
            q,
            factorials: Factorial::new_big(),
        }
    }
}

impl<T: Clone + CheckedAdd + CheckedMul + Integer> Iterator for Exponential<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.sum.clone();
        self.val = self.val.checked_mul(&self.q)?;
        let frac = Ratio::new(T::one(), self.factorials.next()?);
        self.sum = self.sum.checked_add(&frac.checked_mul(&self.val)?)?;
        Some(out)
    }
}

/// Sequence of values converging on the natural logarithm of a rational number calculated using the inverse hyperbolic tangent formula.
///
/// ```text
/// q = 2/1
/// 0, 2/3, 56/81, 842/1215, 53056/76545, 4297606/6200145...
///
/// q = 3/2
/// 0, 1/2, 49/96, 3923/7680, 439391/860160, 21090803/41287680...
/// ```
pub struct NaturalLog<T> {
    sum: Ratio<T>,
    val: Ratio<T>,
    q: Ratio<T>,
    ctr: T,
}

impl NaturalLog<Number> {
    /// Panics if numer/denom <= 0.
    pub fn new(numer: Number, denom: Number) -> Self {
        assert!(Ratio::new(numer, denom) > Ratio::zero());
        let q_minus_one = Ratio::new(numer, denom) - Ratio::one();
        let q_plus_one = Ratio::new(numer, denom) + Ratio::one();
        Self {
            sum: Ratio::zero(),
            val: q_minus_one / q_plus_one,
            q: q_minus_one / q_plus_one,
            ctr: 1,
        }
    }

    /// Panics if q <= 0.
    pub fn from_ratio(q: Ratio<Number>) -> Self {
        assert!(q > Ratio::zero());
        let q_minus_one = q - Ratio::one();
        let q_plus_one = q + Ratio::one();
        Self {
            sum: Ratio::zero(),
            val: q_minus_one / q_plus_one,
            q: q_minus_one / q_plus_one,
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
        let q = Ratio::new(BigInt::from(numer), BigInt::from(denom));
        assert!(q > Ratio::zero());
        Self {
            sum: Ratio::zero(),
            val: (q.clone() - Ratio::one()) / (q.clone() + Ratio::one()),
            q: (q.clone() - Ratio::one()) / (q + Ratio::one()),
            ctr: BigInt::one(),
        }
    }

    /// Panics if q <= 0.
    pub fn from_ratio_big<G: Clone>(q: Ratio<G>) -> Self
    where
        BigInt: From<G>,
    {
        let x = Ratio::new(
            BigInt::from(q.numer().clone()),
            BigInt::from(q.denom().clone()),
        );
        assert!(x > Ratio::zero());
        Self {
            sum: Ratio::zero(),
            val: (x.clone() - Ratio::one()) / (x.clone() + Ratio::one()),
            q: (x.clone() - Ratio::one()) / (x + Ratio::one()),
            ctr: BigInt::one(),
        }
    }
}

impl<T: Clone + CheckedAdd + CheckedMul + Integer> Iterator for NaturalLog<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.sum.clone();

        let frac = Ratio::new(T::one(), self.ctr.clone());

        self.sum = self.sum.checked_add(&frac.checked_mul(&self.val)?)?;

        // Multiply twice to get an odd power
        self.val = self.val.checked_mul(&self.q)?;
        self.val = self.val.checked_mul(&self.q)?;

        self.ctr.incr()?;
        self.ctr.incr()?;

        // Times two
        Some(out.clone() + out)
    }
}

crate::sample_sequences!(
    Exponential::new_big(1,1); // converges on e
    Exponential::new_big(3,2);
    NaturalLog::new_big(2,1);
    NaturalLog::new_big(5,3);
);
