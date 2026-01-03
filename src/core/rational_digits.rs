use num::{
    BigInt, CheckedDiv, CheckedMul, CheckedSub, Integer, One, PrimInt, Signed, Zero,
    rational::Ratio,
};

/// Decimal digits of a fraction between one and zero. The infinite trailing zeroes are included.
pub struct DecimalDigits<T> {
    denom: T,
    remdr: T,
    ten: T,
}

impl<T: PrimInt + Integer> DecimalDigits<T> {
    pub fn new(numer: T, denom: T) -> Self {
        assert!(numer >= T::zero());
        assert!(denom > T::zero());
        assert!(denom > numer);
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
        Self::new(q.numer().clone(), q.denom().clone())
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
        assert!(den > num);
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
        Self::new_big(num, den)
    }
}

impl<T: CheckedDiv + CheckedSub + CheckedMul> Iterator for DecimalDigits<T> {
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

crate::check_sequences!(
    DecimalDigits::new(665857, 941664), 0, 20, [0,7,0,7,1,0,6,7,8,1,1,8,7,3,4,4,9,5,5,3];
);
