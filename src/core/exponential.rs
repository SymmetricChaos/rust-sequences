use crate::core::Factorials;
use num::{
    BigInt, CheckedAdd, CheckedMul, CheckedSub, Integer, One, PrimInt, Zero, rational::Ratio,
};

/// The partial sums of the Taylor series form of the exponential function evaluated at n/d.
pub struct Exponential<T> {
    sum: Ratio<T>,
    val: Ratio<T>,
    x: Ratio<T>,
    factorials: Factorials<T>,
}

impl<T: PrimInt + Integer> Exponential<T> {
    pub fn new(numer: T, denom: T) -> Self {
        Self {
            sum: Ratio::one(),
            val: Ratio::one(),
            x: Ratio::new(numer, denom),
            factorials: Factorials::new(),
        }
    }
}

impl Exponential<BigInt> {
    pub fn new_big<G>(numer: G, denom: G) -> Self
    where
        BigInt: From<G>,
    {
        Self {
            sum: Ratio::one(),
            val: Ratio::one(),
            x: Ratio::new(BigInt::from(numer), BigInt::from(denom)),
            factorials: Factorials::new_big(),
        }
    }
}

impl<T: Clone + Integer + One + CheckedAdd + CheckedMul> Iterator for Exponential<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.sum.clone();
        self.val = self.val.checked_mul(&self.x)?;
        let frac = Ratio::new(T::one(), self.factorials.next()?);
        self.sum = self.sum.checked_add(&frac.checked_mul(&self.val)?)?;
        Some(out)
    }
}

/// The partial sums of the Taylor series form of the natural logarithm evaluated at n/d. Only valid for values between 0 and 2.
pub struct NaturalLog<T> {
    sum: Ratio<T>,
    val: Ratio<T>,
    x: Ratio<T>,
    ctr: T,
    add: bool,
}

impl<T: PrimInt + Integer> NaturalLog<T> {
    /// Panics if numer/denom <= 0.
    /// Panics if numer/denom > 2.
    pub fn new(numer: T, denom: T) -> Self {
        assert!(Ratio::new(numer, denom) > Ratio::zero());
        assert!(Ratio::new(numer, denom) <= Ratio::one() + Ratio::one());
        Self {
            sum: Ratio::zero(),
            val: Ratio::one(),
            x: Ratio::new(numer, denom) - Ratio::one(),
            ctr: T::one(),
            add: true,
        }
    }
}

impl NaturalLog<BigInt> {
    /// Panics if numer/denom <= 0.
    /// Panics if numer/denom > 2.
    pub fn new_big<G>(numer: G, denom: G) -> Self
    where
        BigInt: From<G>,
    {
        let x = Ratio::new(BigInt::from(numer), BigInt::from(denom));
        assert!(x > Ratio::zero());
        assert!(x <= Ratio::one() + Ratio::one());
        Self {
            sum: Ratio::zero(),
            val: Ratio::one(),
            x: x - Ratio::one(),
            ctr: BigInt::one(),
            add: true,
        }
    }
}

impl<T: Clone + Integer + One + CheckedAdd + CheckedSub + CheckedMul> Iterator for NaturalLog<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.sum.clone();
        self.val = self.val.checked_mul(&self.x)?;
        let frac = Ratio::new(T::one(), self.ctr.clone());
        if self.add {
            self.sum = self.sum.checked_add(&frac.checked_mul(&self.val)?)?;
        } else {
            self.sum = self.sum.checked_sub(&frac.checked_mul(&self.val)?)?;
        }
        self.ctr = self.ctr.checked_add(&T::one())?;
        self.add = !self.add;

        Some(out)
    }
}

crate::print_values!(
    Exponential::new_big(1,1), 0, 15; // converges on e
    Exponential::new(1,1), 0, 15; // converges on e
    NaturalLog::new(3,2), 0, 10;
);
