use num::{
    BigInt, CheckedAdd, CheckedMul, CheckedSub, Integer, One, Signed, Zero, rational::Ratio,
};

use crate::Number;

/// The Farey sequence of a given order, k.
///
/// ```text
/// k = 4
/// 0, 1/4, 1/3, 1/2, 2/3, 3/4, 1
///
/// k = 7
/// 1, 6/7, 5/6, 4/5, 3/4, 5/7, 2/3, 3/5, 4/7, 1/2, 3/7, 2/5, 1/3, 2/7...
/// ```
pub struct Farey<T> {
    n0: T,
    d0: T,
    n1: T,
    d1: T,
    k: T,
}

impl Farey<Number> {
    /// The ascending Farey sequence of order k. Panics if k is not positive.
    pub fn new(k: Number) -> Self {
        assert!(k.is_positive());
        Self {
            n0: 0,
            d0: 1,
            n1: 1,
            d1: k,
            k,
        }
    }

    /// The ascending Farey sequence of order k. Panics if k is not positive.
    pub fn new_ascending(k: Number) -> Self {
        assert!(k.is_positive());
        Self {
            n0: 0,
            d0: 1,
            n1: 1,
            d1: k,
            k,
        }
    }

    /// The descending Farey sequence of order k. Panics if k is not positive.
    pub fn new_descending(k: Number) -> Self {
        assert!(k.is_positive());
        Self {
            n0: 1,
            d0: 1,
            n1: k - 1,
            d1: k,
            k,
        }
    }
}

impl Farey<BigInt> {
    /// The ascending Farey sequence of order k. Panics if k is not positive.
    pub fn new_big<G>(k: G) -> Self
    where
        BigInt: From<G>,
    {
        let k = BigInt::from(k);
        assert!(k.is_positive());
        Self {
            n0: BigInt::zero(),
            d0: BigInt::one(),
            n1: BigInt::one(),
            d1: k.clone(),
            k,
        }
    }

    /// The ascending Farey sequence of order k. Panics if k is not positive.
    pub fn new_big_ascending<G>(k: G) -> Self
    where
        BigInt: From<G>,
    {
        let k = BigInt::from(k);
        assert!(k.is_positive());
        Self {
            n0: BigInt::zero(),
            d0: BigInt::one(),
            n1: BigInt::one(),
            d1: k.clone(),
            k,
        }
    }

    /// The descending Farey sequence of order k. Panics if k is not positive.
    pub fn new_big_descending<G>(k: G) -> Self
    where
        BigInt: From<G>,
    {
        let k = BigInt::from(k);
        assert!(k.is_positive());
        Self {
            n0: BigInt::one(),
            d0: BigInt::one(),
            n1: &k - 1,
            d1: k.clone(),
            k,
        }
    }
}

impl<T: Clone + CheckedAdd + CheckedMul + CheckedSub + Integer> Iterator for Farey<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n0 > self.d0 || self.n0 < T::zero() {
            return None;
        }

        let out = Ratio::new(self.n0.clone(), self.d0.clone());
        let t = self.k.checked_add(&self.d0)? / self.d1.clone();
        let n2 = t.checked_mul(&self.n1)?.checked_sub(&self.n0)?;
        let d2 = t.checked_mul(&self.d1)?.checked_sub(&self.d0)?;
        self.n0 = self.n1.clone();
        self.d0 = self.d1.clone();
        self.n1 = n2;
        self.d1 = d2;

        Some(out)
    }
}

crate::sample_sequences!(
    Farey::new_big(3);
    Farey::new_big(4);
    Farey::new_big_descending(3);
    Farey::new_big_descending(7);
    Farey::new_descending(7);
    Farey::new(4);
);
