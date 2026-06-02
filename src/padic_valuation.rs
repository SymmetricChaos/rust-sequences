use crate::{
    Number,
    core::{rationals::Rationals, traits::Increment},
    utils::padic::{padic_abs, padic_valuation},
};
use num::{BigInt, CheckedMul, One, rational::Ratio};

/// The k-adic valuation of each positive integer. When k is prime it is a p-adic valuation.
///
/// ```text
/// For k = 2
/// 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0...
/// ```
pub struct PadicValuation<T> {
    p: T,
    ctr: T,
}

impl PadicValuation<Number> {
    pub fn new(p: Number) -> Self {
        Self { p, ctr: 1 }
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

impl Iterator for PadicValuation<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let val = padic_valuation(&self.ctr, &self.p);
        self.ctr.incr()?;
        Some(val)
    }
}

impl Iterator for PadicValuation<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let val = padic_valuation(&self.ctr, &self.p);
        self.ctr.incr()?;
        Some(val)
    }
}

/// The k-adic absolute value of each positive integer. When k is prime it is a p-adic absolute value.
///
/// ```text
/// For k = 2
/// 1, 1/2, 1, 1/4, 1, 1/2, 1, 1/8...
/// ```
pub struct PadicAbs<T> {
    p: T,
    ctr: T,
}

impl PadicAbs<Number> {
    pub fn new(p: Number) -> Self {
        Self { p, ctr: 1 }
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

impl Iterator for PadicAbs<Number> {
    type Item = Ratio<Number>;

    fn next(&mut self) -> Option<Self::Item> {
        let val = padic_abs(&self.ctr, &self.p);
        self.ctr.incr()?;
        Some(val)
    }
}

impl Iterator for PadicAbs<BigInt> {
    type Item = Ratio<BigInt>;

    fn next(&mut self) -> Option<Self::Item> {
        let val = padic_abs(&self.ctr, &self.p);
        self.ctr.incr()?;
        Some(val)
    }
}

/// The k-adic valuation of each positive rational number. When k is prime it is a p-adic valuation.
pub struct PadicValuationRational<T> {
    p: T,
    rationals: Rationals<T>,
}

impl PadicValuationRational<Number> {
    pub fn new(p: Number) -> Self {
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

impl Iterator for PadicValuationRational<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let q = self.rationals.next()?;

        let val_numer = padic_valuation(q.numer(), &self.p);
        let val_denom = padic_valuation(q.denom(), &self.p);

        Some(val_numer - val_denom)
    }
}

impl Iterator for PadicValuationRational<BigInt> {
    type Item = BigInt;

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

impl PadicAbsRational<Number> {
    pub fn new(p: Number) -> Self {
        Self {
            p,
            rationals: Rationals::new_pos(),
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

impl Iterator for PadicAbsRational<Number> {
    type Item = Ratio<Number>;

    fn next(&mut self) -> Option<Self::Item> {
        let q = self.rationals.next()?;

        let val_numer = padic_abs(q.numer(), &self.p);
        let val_denom = padic_abs(q.denom(), &self.p).recip();

        val_numer.checked_mul(&val_denom)
    }
}

impl Iterator for PadicAbsRational<BigInt> {
    type Item = Ratio<BigInt>;

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
