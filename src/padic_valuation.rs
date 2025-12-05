use crate::core::rationals::Rationals;
use num::{BigInt, CheckedAdd, CheckedSub, Integer, One, Signed};

pub fn padic_valuation<T: CheckedAdd + Clone + Integer>(n: &T, p: &T) -> T {
    let mut val = T::zero();
    let mut n = n.clone();
    loop {
        let (q, r) = n.div_rem(&p);
        n = q;
        if r.is_zero() {
            val = val + T::one();
        } else {
            break;
        }
    }
    val
}

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

impl<T: CheckedAdd + Clone + Integer + CheckedSub + Signed> Iterator for PadicValuationRational<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let q = self.rationals.next()?;

        let val_numer = padic_valuation(q.numer(), &self.p);
        let val_denom = padic_valuation(q.denom(), &self.p);

        Some(val_numer - val_denom)
    }
}

crate::print_values!(
    PadicValuationRational::new_big(2), 0, 30;
);

crate::check_sequences!(
    PadicValuation::new(2), 0, 20, [0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, 4, 0, 1, 0, 2];
);
