use std::fmt::Display;

use num::{
    BigInt, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Integer, PrimInt, Zero, rational::Ratio,
};

pub fn integer_digits<T: Integer>(n: T, base: T) -> Vec<T> {
    let mut out = Vec::new();
    let mut n = n;
    while n > T::zero() {
        let (div, rem) = n.div_rem(&base);
        out.push(rem);
        n = div;
    }
    out
}

/// Digits of a fraction in a chosen base. If the fraction is less than one a leading zero is included to represent the integer part. If the fraction terminates infinite trailing zeroes are produced.
pub struct RationalDigits<T> {
    remdr: T,
    denom: T,
    base: T,
    numer: Vec<T>,
    leading_zero: bool,
}

impl<T: PrimInt + Integer + Display> RationalDigits<T> {
    pub fn new(numer: T, denom: T, base: T) -> Self {
        Self::from_ratio(Ratio::new(numer, denom), base)
    }

    /// Decimal digits.
    pub fn new_decimal(numer: T, denom: T) -> Self {
        Self::from_ratio(
            Ratio::new(numer, denom),
            T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one(),
        )
    }

    pub fn from_ratio(q: Ratio<T>, base: T) -> Self {
        assert!(q > Ratio::<T>::zero());
        let f = q.into_raw();
        let numer = integer_digits(f.0, base);
        let remdr = T::zero();
        Self {
            remdr,
            denom: f.1,
            base,
            numer,
            leading_zero: true,
        }
    }
}

impl RationalDigits<BigInt> {
    pub fn new_big<F: Clone + Integer>(numer: F, denom: F, base: F) -> Self
    where
        Ratio<BigInt>: From<Ratio<F>>,
        BigInt: From<F>,
    {
        Self::from_ratio_big(Ratio::new(numer, denom), base)
    }

    /// Decimal digits.
    pub fn new_decimal_big<F: Clone + Integer>(numer: F, denom: F) -> Self
    where
        Ratio<BigInt>: From<Ratio<F>>,
        BigInt: From<F>,
    {
        Self::from_ratio_big(
            Ratio::new(numer, denom),
            F::one()
                + F::one()
                + F::one()
                + F::one()
                + F::one()
                + F::one()
                + F::one()
                + F::one()
                + F::one()
                + F::one(),
        )
    }

    pub fn from_ratio_big<F: Clone + Integer>(q: Ratio<F>, base: F) -> Self
    where
        Ratio<BigInt>: From<Ratio<F>>,
        BigInt: From<F>,
    {
        let q: Ratio<BigInt> = q.into();
        assert!(q > Ratio::<BigInt>::zero());
        let base = BigInt::from(base);
        let f = q.into_raw();
        let numer = integer_digits(f.0, base.clone());
        Self {
            remdr: BigInt::zero(),
            denom: f.1,
            base,
            numer,
            leading_zero: true,
        }
    }
}

impl<T: CheckedDiv + CheckedSub + CheckedMul + CheckedAdd + Zero> Iterator for RationalDigits<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.leading_zero {
            // Ugly loop to remove leading zeroes from the integer part of a value
            loop {
                self.remdr = self
                    .remdr
                    .checked_mul(&self.base)?
                    .checked_add(&self.numer.pop().unwrap_or(T::zero()))?;
                let out = self.remdr.checked_div(&self.denom)?;
                self.remdr = self.remdr.checked_sub(&self.denom.checked_mul(&out)?)?;
                if self.numer.is_empty() || !out.is_zero() {
                    self.leading_zero = false;
                    return Some(out);
                }
            }
        } else {
            self.remdr = self
                .remdr
                .checked_mul(&self.base)? // shift the digits of the remainder
                .checked_add(&self.numer.pop().unwrap_or(T::zero()))?; // pull down the next digit or zero if digits are used up
            let out = self.remdr.checked_div(&self.denom)?; // determine the next digit
            self.remdr = self.remdr.checked_sub(&self.denom.checked_mul(&out)?)?; // subtract from the remainder
            Some(out)
        }
    }
}

crate::check_sequences!(
    RationalDigits::new_decimal(665857, 941664), 0, 20, [0,7,0,7,1,0,6,7,8,1,1,8,7,3,4,4,9,5,5,3];
    RationalDigits::new_decimal(10, 7), 0, 10, [1, 4, 2, 8, 5, 7, 1, 4, 2, 8];
    RationalDigits::new_decimal(46, 3), 0, 10, [1, 5, 3, 3, 3, 3, 3, 3, 3, 3];
    RationalDigits::new_decimal(1, 127), 0, 20, [0, 0, 0, 7, 8, 7, 4, 0, 1, 5, 7, 4, 8, 0, 3, 1, 4, 9, 6, 0]; // check for correct leading zeroes, this is 0.007874...
);
crate::print_values!(
    digits, formatter "{}", sep " ";
    RationalDigits::new(665857, 941664, 2), 0, 20;
    RationalDigits::new(1, 4, 2), 0, 20;
    RationalDigits::new(10, 7, 10), 0, 20;
    RationalDigits::new(301, 3, 10), 0, 20;
    RationalDigits::new(1, 127, 10), 0, 20;
    RationalDigits::new(46, 3, 10), 0, 10;
);
