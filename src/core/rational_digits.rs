use num::{
    BigInt, CheckedDiv, CheckedMul, CheckedSub, Integer, PrimInt, Signed, Zero, rational::Ratio,
};

/// Decimal digits of a fraction between one and zero. The infinite trailing zeroes are included.
pub struct DecimalDigits<T> {
    denom: T,
    remdr: T,
    base: T,
}

impl<T: PrimInt + Integer> DecimalDigits<T> {
    pub fn new(numer: T, denom: T, base: T) -> Self {
        assert!(numer >= T::zero());
        assert!(denom > T::zero());
        assert!(denom > numer);
        let g = numer.gcd(&denom);
        Self {
            denom: denom / g,
            remdr: numer / g,
            base,
        }
    }

    pub fn from_ratio(q: Ratio<T>, base: T) -> Self {
        Self::new(q.numer().clone(), q.denom().clone(), base)
    }
}

impl DecimalDigits<BigInt> {
    pub fn new_big<F>(numer: F, denom: F, base: F) -> Self
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
            base: BigInt::from(base),
        }
    }

    pub fn from_ratio_big<F>(q: Ratio<F>, base: F) -> Self
    where
        BigInt: From<F>,
    {
        let (num, den) = q.into_raw();
        Self::new_big(num, den, base)
    }
}

impl<T: CheckedDiv + CheckedSub + CheckedMul> Iterator for DecimalDigits<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.remdr.checked_div(&self.denom)?;
        self.remdr = self
            .remdr
            .checked_sub(&self.denom.checked_mul(&out)?)?
            .checked_mul(&self.base)?;
        Some(out)
    }
}

crate::check_sequences!(
    DecimalDigits::new(665857, 941664, 10), 0, 20, [0,7,0,7,1,0,6,7,8,1,1,8,7,3,4,4,9,5,5,3];
);
