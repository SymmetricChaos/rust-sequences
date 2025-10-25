use num::{
    BigInt, CheckedDiv, CheckedMul, CheckedSub, FromPrimitive, Integer, One, PrimInt, Signed, Zero,
    rational::Ratio,
};

/// Decimal digits for the fractional part of of a non-negative fraction, preceeded by the whole part.
pub struct DecimalDigits<T> {
    denom: T,
    remdr: T,
    ten: T,
}

impl<T: PrimInt + Integer> DecimalDigits<T> {
    pub fn new(numer: T, denom: T) -> Self {
        assert!(numer >= T::zero());
        assert!(denom > T::zero());
        let g = numer.gcd(&denom);
        Self {
            denom: denom / g,
            remdr: numer / g,
            ten: T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one(),
        }
    }

    pub fn from_ratio(q: Ratio<T>) -> Self {
        assert!(q > Ratio::<T>::zero());
        Self {
            denom: q.denom().clone(),
            remdr: q.numer().clone(),
            ten: T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one()
                + T::one(),
        }
    }
}

impl DecimalDigits<BigInt> {
    pub fn new_big<F>(numer: F, denom: F) -> Self
    where
        BigInt: From<F>,
    {
        let num = BigInt::from(numer).abs();
        let den = BigInt::from(denom).abs();
        assert!(num >= BigInt::zero());
        assert!(den > BigInt::zero());
        let g = num.gcd(&den);
        Self {
            denom: den / &g,
            remdr: num / &g,
            ten: BigInt::one()
                + BigInt::one()
                + BigInt::one()
                + BigInt::one()
                + BigInt::one()
                + BigInt::one()
                + BigInt::one()
                + BigInt::one()
                + BigInt::one()
                + BigInt::one(),
        }
    }

    pub fn from_ratio_big<F>(q: Ratio<F>) -> Self
    where
        BigInt: From<F>,
    {
        let (num, den) = q.into_raw();
        Self {
            denom: BigInt::from(den).abs(),
            remdr: BigInt::from(num).abs(),
            ten: BigInt::one()
                + BigInt::one()
                + BigInt::one()
                + BigInt::one()
                + BigInt::one()
                + BigInt::one()
                + BigInt::one()
                + BigInt::one()
                + BigInt::one()
                + BigInt::one(),
        }
    }
}

impl<T: CheckedDiv + CheckedSub + CheckedMul + FromPrimitive> Iterator for DecimalDigits<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.remdr.checked_div(&self.denom)?;
        self.remdr = self
            .remdr
            .checked_sub(&self.denom.checked_mul(&out)?)?
            .checked_mul(&self.ten)?;
        Some(out)
    }
}

crate::print_values!(
    digits, formatter "{}", sep " ";
    DecimalDigits::new(665857, 941664), 0, 20; // should be close to 0.70710678118
    DecimalDigits::new(127, 11), 0, 20;
);
