use num::{
    BigInt, CheckedDiv, CheckedMul, CheckedSub, FromPrimitive, Integer, PrimInt, Zero,
    rational::Ratio,
};

/// Decimal digits for the fractional part of of a non-negative fraction, preceeded by the whole part.
pub struct DecimalDigits<T> {
    denom: T,
    remdr: T,
}

impl<T: PrimInt + Integer> DecimalDigits<T> {
    pub fn new_prim(numer: T, denom: T) -> Self {
        assert!(numer >= T::zero());
        assert!(denom > T::zero());
        let g = numer.gcd(&denom);
        Self {
            denom: denom / g,
            remdr: numer / g,
        }
    }

    pub fn from_prim_ratio(q: Ratio<T>) -> Self {
        assert!(q > Ratio::<T>::zero());
        Self {
            denom: q.denom().clone(),
            remdr: q.numer().clone(),
        }
    }
}

impl DecimalDigits<BigInt> {
    pub fn new<F: Zero + Ord>(numer: F, denom: F) -> Self
    where
        BigInt: From<F>,
    {
        assert!(numer >= F::zero());
        assert!(denom > F::zero());
        let num = BigInt::from(numer);
        let den = BigInt::from(denom);
        let g = num.gcd(&den);
        Self {
            denom: den / &g,
            remdr: num / &g,
        }
    }

    pub fn from_ratio(q: Ratio<BigInt>) -> Self {
        assert!(q > Ratio::<BigInt>::zero());
        Self {
            denom: q.denom().clone(),
            remdr: q.numer().clone(),
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
            .checked_mul(&T::from_u32(10).unwrap())?;
        Some(out)
    }
}

crate::print_values!(
    digits, formatter "{}", sep " ";
    DecimalDigits::new_prim(665857, 941664), 0, 20; // should be close to 0.70710678118
    DecimalDigits::new_prim(127, 11), 0, 20;
);
