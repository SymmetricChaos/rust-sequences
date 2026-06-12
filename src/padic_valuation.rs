use crate::{
    Number,
    core::{rationals::Rationals, traits::Increment},
    utils::padic::{padic_abs, padic_valuation},
};
use num::{BigInt, CheckedAdd, CheckedMul, CheckedSub, Integer, One, rational::Ratio};

/// The k-adic valuation of each positive integer. When k is prime it is a p-adic valuation.
///
/// ```text
/// k = 2
/// 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, 4, 0, 1, 0, 2, 0, 1, 0...
///
/// k = 3
/// 0, 0, 1, 0, 0, 1, 0, 0, 2, 0, 0, 1, 0, 0, 1, 0, 0, 2, 0, 0, 1, 0, 0...
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

#[cfg(feature = "big_int")]
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

impl<T: Clone + CheckedAdd + Integer> Iterator for PadicValuation<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let val = padic_valuation(&self.ctr, &self.p);
        self.ctr.incr()?;
        Some(val)
    }
}

/// The k-adic absolute value of each positive integer. When k is prime it is a p-adic absolute value.
///
/// ```text
/// k = 2
/// 1, 1/2, 1, 1/4, 1, 1/2, 1, 1/8, 1, 1/2, 1, 1/4, 1, 1/2, 1, 1/16, 1...
///
/// k = 3
/// 1, 1, 1/3, 1, 1, 1/3, 1, 1, 1/9, 1, 1, 1/3, 1, 1, 1/3, 1, 1, 1/9, 1...
/// ```
pub struct PadicAbs<T> {
    p: T,
    ctr: T,
}

impl PadicAbs<Number> {
    pub fn new(p: Number) -> Self {
        Self { p, ctr: 1 }
    }

    pub fn numers(p: Number) -> impl Iterator<Item = Number> {
        Self::new(p).map(|q| *q.numer())
    }

    pub fn denoms(p: Number) -> impl Iterator<Item = Number> {
        Self::new(p).map(|q| *q.denom())
    }
}

#[cfg(feature = "big_int")]
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

    pub fn numers<T>(p: T) -> impl Iterator<Item = BigInt>
    where
        BigInt: From<T>,
    {
        Self::new_big(p).map(|q| q.numer().clone())
    }

    pub fn denoms<T>(p: T) -> impl Iterator<Item = BigInt>
    where
        BigInt: From<T>,
    {
        Self::new_big(p).map(|q| q.denom().clone())
    }
}

impl<T: Clone + CheckedAdd + Integer> Iterator for PadicAbs<T> {
    type Item = Ratio<T>;

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

#[cfg(feature = "big_int")]
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

impl<T: Clone + CheckedAdd + CheckedSub + Integer> Iterator for PadicValuationRational<T> {
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

impl PadicAbsRational<Number> {
    pub fn new(p: Number) -> Self {
        Self {
            p,
            rationals: Rationals::new_pos(),
        }
    }

    pub fn numers(p: Number) -> impl Iterator<Item = Number> {
        Self::new(p).map(|q| *q.numer())
    }

    pub fn denoms(p: Number) -> impl Iterator<Item = Number> {
        Self::new(p).map(|q| *q.denom())
    }
}

#[cfg(feature = "big_int")]
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

    pub fn numers<T>(p: T) -> impl Iterator<Item = BigInt>
    where
        BigInt: From<T>,
    {
        Self::new_big(p).map(|q| q.numer().clone())
    }

    pub fn denoms<T>(p: T) -> impl Iterator<Item = BigInt>
    where
        BigInt: From<T>,
    {
        Self::new_big(p).map(|q| q.denom().clone())
    }
}

impl<T: Clone + CheckedAdd + CheckedMul + CheckedSub + Integer> Iterator for PadicAbsRational<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let q = self.rationals.next()?;

        let val_numer = padic_abs(q.numer(), &self.p);
        let val_denom = padic_abs(q.denom(), &self.p).recip();

        val_numer.checked_mul(&val_denom)
    }
}

crate::check_sequences!(
    PadicValuation::new(2), [0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, 4, 0, 1, 0, 2];
    PadicAbs::new(2), ["1", "1/2", "1", "1/4", "1", "1/2", "1", "1/8"];
);

crate::sample_sequences!(
    PadicValuation::new(2);
    PadicValuation::new(3);
    PadicAbs::new(2);
    PadicAbs::new(3);
);
