use crate::{Number, core::traits::Increment};
use num::{BigInt, CheckedAdd, CheckedSub, Integer, One, Signed, Zero, rational::Ratio};

/// The non-negative rational numbers in anti-diagonal order
pub struct Rationals<T> {
    numer: T,
    denom: T,
    row: T,
}

impl Rationals<Number> {
    /// The non-negative rationals
    pub fn new() -> Self {
        Self {
            numer: 1,
            denom: 1,
            row: 0,
        }
    }

    /// The positive rationals
    pub fn new_pos() -> Self {
        Self {
            numer: 1,
            denom: 1,
            row: 1,
        }
    }
}

#[cfg(feature = "big_int")]
impl Rationals<BigInt> {
    /// The non-negative rationals
    pub fn new_big() -> Self {
        Self {
            numer: BigInt::one(),
            denom: BigInt::one(),
            row: BigInt::zero(),
        }
    }

    /// The positive rationals
    pub fn new_big_pos() -> Self {
        Self {
            numer: BigInt::one(),
            denom: BigInt::one(),
            row: BigInt::one(),
        }
    }
}

impl<T: CheckedAdd + CheckedSub + Clone + Ord + Integer> Iterator for Rationals<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row.is_zero() {
            self.row = self.row.checked_add(&T::one())?;
            return Some(Ratio::zero());
        }

        // new_raw() justified because gcd is always checked before we get here
        let out = Ratio::new_raw(self.numer.clone(), self.denom.clone());

        loop {
            self.numer = self.numer.checked_sub(&T::one())?;
            self.denom = self.denom.checked_add(&T::one())?;
            if self.numer.is_zero() {
                self.row.incr()?;
                self.numer = self.row.clone();
                self.denom = T::one();
            }
            if self.numer.gcd(&self.denom) == T::one() {
                break;
            }
        }

        Some(out)
    }
}

/// All rational numbers, starting from zero, with the positive rationals in anti-diagonal order each followed by its negative.
pub struct SignedRationals<T> {
    numer: T,
    denom: T,
    row: T,
    positive: bool,
}

impl SignedRationals<Number> {
    pub fn new() -> Self {
        Self {
            numer: 1,
            denom: 1,
            row: 0,
            positive: true,
        }
    }
}

#[cfg(feature = "big_int")]
impl SignedRationals<BigInt> {
    pub fn new_big() -> Self {
        Self {
            numer: BigInt::one(),
            denom: BigInt::one(),
            row: BigInt::zero(),
            positive: true,
        }
    }
}

impl<T: CheckedAdd + CheckedSub + Clone + Ord + Integer + Signed> Iterator for SignedRationals<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row.is_zero() {
            self.row.incr()?;
            return Some(Ratio::zero());
        }

        if !self.positive {
            self.positive = true;
            // new_raw() justified because gcd is always checked before we get here
            let out = Some(Ratio::new_raw(-self.numer.clone(), self.denom.clone()));
            loop {
                self.numer = self.numer.checked_sub(&T::one())?;
                self.denom = self.denom.checked_add(&T::one())?;
                if self.numer.is_zero() {
                    self.row.incr()?;
                    self.numer = self.row.clone();
                    self.denom = T::one();
                }
                if self.numer.gcd(&self.denom) == T::one() {
                    return out;
                }
            }
        } else {
            self.positive = false;
            return Some(Ratio::new_raw(self.numer.clone(), self.denom.clone()));
        };
    }
}

crate::check_sequences!(
    Rationals::new(), ["0", "1", "2", "1/2", "3", "1/3", "4", "3/2", "2/3", "1/4", "5", "1/5", "6", "5/2", "4/3", "3/4", "2/5", "1/6", "7", "5/3"];
    SignedRationals::new(), ["0", "1", "-1", "2", "-2", "1/2", "-1/2", "3", "-3", "1/3", "-1/3", "4", "-4", "3/2", "-3/2", "2/3", "-2/3", "1/4", "-1/4", "5"];
);
