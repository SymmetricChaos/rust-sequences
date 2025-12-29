use num::{
    BigInt, CheckedDiv, CheckedMul, CheckedSub, Integer, One, PrimInt, Signed, Zero,
    rational::Ratio,
};

/// Bits of a fraction between one and zero. The indefinite training zeroes are included.
pub struct RationalBits<T> {
    denom: T,
    remdr: T,
    two: T,
}

impl<T: PrimInt + Integer> RationalBits<T> {
    pub fn new(numer: T, denom: T) -> Self {
        assert!(numer >= T::zero());
        assert!(denom > T::zero());
        assert!(denom > numer);
        let g = numer.gcd(&denom);
        Self {
            denom: denom / g,
            remdr: numer / g,
            two: T::one() + T::one(),
        }
    }

    pub fn from_ratio(q: Ratio<T>) -> Self {
        Self::new(q.numer().clone(), q.denom().clone())
    }
}

impl RationalBits<BigInt> {
    pub fn new_big<F>(numer: F, denom: F) -> Self
    where
        BigInt: From<F>,
    {
        let num = BigInt::from(numer).abs();
        let den = BigInt::from(denom).abs();
        assert!(num >= BigInt::zero());
        assert!(den > BigInt::zero());
        assert!(den > num);
        let g = num.gcd(&den);
        Self {
            denom: den / &g,
            remdr: num / &g,
            two: BigInt::one() + BigInt::one(),
        }
    }

    pub fn from_ratio_big<F>(q: Ratio<F>) -> Self
    where
        BigInt: From<F>,
    {
        let (num, den) = q.into_raw();
        Self::new_big(num, den)
    }
}

impl<T: CheckedDiv + CheckedSub + CheckedMul> Iterator for RationalBits<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.remdr.checked_div(&self.denom)?;
        self.remdr = self
            .remdr
            .checked_sub(&self.denom.checked_mul(&out)?)?
            .checked_mul(&self.two)?;
        Some(out)
    }
}

crate::print_values!(
    digits, formatter "{}", sep "";
    RationalBits::new(665857, 941664), 0, 20;
    RationalBits::new(1, 4), 0, 20;
);
