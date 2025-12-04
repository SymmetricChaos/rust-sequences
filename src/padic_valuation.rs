use crate::core::rationals::Rationals;
use num::{BigInt, CheckedAdd, CheckedSub, Integer, One, Signed};

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
        let mut val = T::zero();
        let mut n = self.ctr.clone();
        loop {
            let (q, r) = n.div_rem(&self.p);
            n = q;
            if r.is_zero() {
                val = val.checked_add(&T::one())?;
            } else {
                break;
            }
        }
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
            rationals: Rationals::<T>::new(),
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
            rationals: Rationals::new_big(),
        }
    }
}

impl<T: CheckedAdd + Clone + Integer + CheckedSub> Iterator for PadicValuationRational<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let q = self.rationals.next()?;
        let mut numer = q.numer().clone();
        let mut denom = q.denom().clone();

        let mut val_numer = T::zero();
        let mut val_denom = T::zero();

        loop {
            let (q, r) = numer.div_rem(&self.p);
            numer = q;
            if r.is_zero() {
                val_numer = val_numer.checked_add(&T::one())?;
            } else {
                break;
            }
        }

        loop {
            let (q, r) = denom.div_rem(&self.p);
            denom = q;
            if r.is_zero() {
                val_denom = val_denom.checked_add(&T::one())?;
            } else {
                break;
            }
        }

        Some(val_numer - val_denom)
    }
}

crate::print_values!(
    PadicValuation::new(3), 0, 30;
);

crate::print_values!(
    rational_pairs, formatter "{:?}", sep " ";
    PadicValuationRational::new_big(3).zip(Rationals::new_big().map(|n| n.to_string())), 0, 40;
);
