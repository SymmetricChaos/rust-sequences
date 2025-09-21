use crate::core::Factorials;
use num::{BigInt, CheckedAdd, CheckedMul, Integer, One, PrimInt, rational::Ratio};

/// The partial sums of the Taylor series form of the exponential function evaluated at x.
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

crate::print_values!(
    Exponential::new_big(1,1), 0, 15; // converges on e
    Exponential::new(1,1), 0, 15; // converges on e
);
