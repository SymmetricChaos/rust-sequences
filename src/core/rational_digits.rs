use num::{
    BigInt, CheckedDiv, CheckedMul, CheckedSub, FromPrimitive, Integer, PrimInt, Zero,
    rational::Ratio,
};

/// Decimal digits of a non-negative fraction.
pub struct RationalDigits<T> {
    den: T,
    rem: T,
}

impl<T: PrimInt + Integer> RationalDigits<T> {
    pub fn new_prim(num: T, den: T) -> Self {
        assert!(num >= T::zero());
        assert!(den > T::zero());
        let g = num.gcd(&den);
        Self {
            den: den / g,
            rem: num / g,
        }
    }

    pub fn from_prim_ratio(q: Ratio<T>) -> Self {
        assert!(q > Ratio::<T>::zero());
        Self {
            den: q.denom().clone(),
            rem: q.numer().clone(),
        }
    }
}

impl RationalDigits<BigInt> {
    pub fn new<F: Zero + Ord>(num: F, den: F) -> Self
    where
        BigInt: From<F>,
    {
        assert!(num >= F::zero());
        assert!(den > F::zero());
        let num = BigInt::from(num);
        let den = BigInt::from(den);
        let g = num.gcd(&den);
        Self {
            den: den / &g,
            rem: num / &g,
        }
    }

    pub fn from_ratio(q: Ratio<BigInt>) -> Self {
        assert!(q > Ratio::<BigInt>::zero());
        Self {
            den: q.denom().clone(),
            rem: q.numer().clone(),
        }
    }
}

impl<T: CheckedDiv + CheckedSub + CheckedMul + FromPrimitive> Iterator for RationalDigits<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.rem.checked_div(&self.den)?;
        self.rem = self
            .rem
            .checked_sub(&self.den.checked_mul(&out)?)?
            .checked_mul(&T::from_i32(10).unwrap())?;
        Some(out)
    }
}

crate::print_values!(
    digits, formatter "{}", sep " ";
    RationalDigits::new_prim(665857, 941664), 0, 20; // should be close to 0.70710678118
    RationalDigits::new_prim(127, 11), 0, 20;
);
