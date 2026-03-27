use crate::{
    core::rationals::Rationals,
    utils::padic::{padic_abs, padic_valuation},
};
use num::{BigInt, CheckedAdd, CheckedMul, CheckedSub, Integer, One, Signed, rational::Ratio};

/// The k-adic valuation of each positive integer. When k is prime it is a p-adic valuation.
pub struct PadicValuation<T> {
    p: T,
    ctr: T,
}

impl<T: CheckedAdd + Clone + Integer + One> PadicValuation<T> {
    pub fn new(p: T) -> Self {
        Self { p, ctr: T::one() }
    }
}

impl PadicValuation<BigInt> {
    pub fn new_big<T>(p: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            p: BigInt::from(p),
            ctr: BigInt::one(),
        }
    }
}

impl<T: CheckedAdd + Clone + Integer> Iterator for PadicValuation<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let val = padic_valuation(&self.ctr, &self.p);
        self.ctr = self.ctr.checked_add(&T::one())?;
        Some(val)
    }
}

/// The k-adic absolute value of each positive integer. When k is prime it is a p-adic absolute value.
pub struct PadicAbs<T> {
    p: T,
    ctr: T,
}

impl<T: CheckedAdd + Clone + Integer + One> PadicAbs<T> {
    pub fn new(p: T) -> Self {
        Self { p, ctr: T::one() }
    }
}

impl PadicAbs<BigInt> {
    pub fn new_big<T>(p: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            p: BigInt::from(p),
            ctr: BigInt::one(),
        }
    }
}

impl<T: CheckedAdd + Clone + Integer> Iterator for PadicAbs<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let val = padic_abs(&self.ctr, &self.p);
        self.ctr = self.ctr.checked_add(&T::one())?;
        Some(val)
    }
}

/// The k-adic valuation of each positive rational number. When k is prime it is a p-adic valuation.
pub struct PadicValuationRational<T> {
    p: T,
    rationals: Rationals<T>,
}

impl<T: CheckedAdd + Clone + Integer + One + Signed + CheckedSub> PadicValuationRational<T> {
    pub fn new(p: T) -> Self {
        Self {
            p,
            rationals: Rationals::new_pos(),
        }
    }
}

impl PadicValuationRational<BigInt> {
    pub fn new_big<T>(p: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            p: BigInt::from(p),
            rationals: Rationals::new_big_pos(),
        }
    }
}

impl<T: CheckedAdd + Clone + Integer + CheckedSub + Signed> Iterator for PadicValuationRational<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let q = self.rationals.next()?;

        let val_numer = padic_valuation(q.numer(), &self.p);
        let val_denom = padic_valuation(q.denom(), &self.p);

        Some(val_numer - val_denom)
    }
}

/// The k-adic valuation of each positive rational number. When k is prime it is a p-adic valuation.
pub struct PadicAbsRational<T> {
    p: T,
    rationals: Rationals<T>,
}

impl<T: CheckedAdd + Clone + Integer + One + Signed + CheckedSub> PadicAbsRational<T> {
    pub fn new(p: T) -> Self {
        Self {
            p,
            rationals: Rationals::<T>::new_pos(),
        }
    }
}

impl PadicAbsRational<BigInt> {
    pub fn new_big<T>(p: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            p: BigInt::from(p),
            rationals: Rationals::new_big_pos(),
        }
    }
}

impl<T: CheckedMul + CheckedAdd + Clone + Integer + CheckedSub + Signed> Iterator
    for PadicAbsRational<T>
{
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let q = self.rationals.next()?;

        let val_numer = padic_abs(q.numer(), &self.p);
        let val_denom = padic_abs(q.denom(), &self.p).recip();

        val_numer.checked_mul(&val_denom)
    }
}

crate::print_sequences!(
    PadicValuationRational::new_big(2), 30;
);

crate::check_sequences!(
    PadicValuation::new(2), [0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, 4, 0, 1, 0, 2];
    PadicAbs::new(2), ["1", "1/2", "1", "1/4", "1", "1/2", "1", "1/8"];
);
