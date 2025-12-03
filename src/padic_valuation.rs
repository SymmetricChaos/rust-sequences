use num::{BigInt, CheckedAdd, Integer, One};

/// The p-adic valuation of each positive integer.
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
        let mut r = T::zero();
        loop {
            (n, r) = n.div_rem(&self.p);
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
