use crate::core::rationals::Rationals;
use num::{
    BigInt, CheckedAdd, CheckedMul, CheckedSub, Integer, One, Signed, Zero, rational::Ratio,
};

pub trait Padic {
    fn padic_abs(&self, p: u32) -> Ratio<BigInt>;
    fn padic_val(&self, p: u32) -> BigInt;
}

impl Padic for BigInt {
    fn padic_abs(&self, p: u32) -> Ratio<BigInt> {
        if self.is_zero() {
            return Ratio::zero();
        }
        let mut denom = BigInt::one();
        let p = BigInt::from(p);
        let mut n = self.clone();
        loop {
            let (q, r) = n.div_rem(&p);
            n = q;
            if r.is_zero() {
                denom = denom * p.clone();
            } else {
                break;
            }
        }
        Ratio::new(BigInt::one(), denom)
    }

    fn padic_val(&self, p: u32) -> BigInt {
        let mut val = BigInt::zero();
        let mut n = self.clone();
        let p = BigInt::from(p);
        loop {
            let (q, r) = n.div_rem(&p);
            n = q;
            if r.is_zero() {
                val = val + BigInt::one();
            } else {
                break;
            }
        }
        val
    }
}

impl Padic for Ratio<BigInt> {
    fn padic_abs(&self, p: u32) -> Ratio<BigInt> {
        self.numer().padic_abs(p) * self.denom().padic_abs(p).recip()
    }

    fn padic_val(&self, p: u32) -> BigInt {
        self.numer().padic_val(p) - self.denom().padic_val(p)
    }
}

/// k-adic valuation of an integer
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

/// k-adic absolute value of an integer
pub fn padic_abs<T: CheckedAdd + Clone + Integer>(n: &T, p: &T) -> Ratio<T> {
    if n.is_zero() {
        return Ratio::zero();
    }
    let mut denom = T::one();
    let mut n = n.clone();
    loop {
        let (q, r) = n.div_rem(&p);
        n = q;
        if r.is_zero() {
            denom = denom * p.clone();
        } else {
            break;
        }
    }
    Ratio::new(T::one(), denom)
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
    PadicValuationRational::new_big(2), 0, 30;
    PadicAbs::new(2), 0, 30;
);

crate::check_sequences!(
    PadicValuation::new(2), [0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, 4, 0, 1, 0, 2];
);
