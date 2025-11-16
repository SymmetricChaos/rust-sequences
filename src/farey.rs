use num::{BigInt, BigRational, One, Signed, Zero, rational::Ratio};

/// The Farey sequence with selectable order.
/// For instance Farey::new_big(4) produces
/// 0, 1/4, 1/3, 1/2, 2/3, 3/4, 1
pub struct Farey {
    n0: BigInt,
    d0: BigInt,
    n1: BigInt,
    d1: BigInt,
    k: BigInt,
}

impl Farey {
    /// The ascending Farey sequence of order k. Panics if k is not positive.
    pub fn new_big<T>(k: T) -> Self
    where
        BigInt: From<T>,
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
    pub fn new_big_ascending<T>(k: T) -> Self
    where
        BigInt: From<T>,
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
    pub fn new_big_descending<T>(k: T) -> Self
    where
        BigInt: From<T>,
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

impl Iterator for Farey {
    type Item = BigRational;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n0 > self.d0 || self.n0.is_negative() {
            return None;
        }

        let out = Ratio::new(self.n0.clone(), self.d0.clone());
        let t = (&self.k + &self.d0) / &self.d1;
        let n2 = &t * &self.n1 - &self.n0;
        let d2 = &t * &self.d1 - &self.d0;
        self.n0 = self.n1.clone();
        self.d0 = self.d1.clone();
        self.n1 = n2;
        self.d1 = d2;

        Some(out)
    }
}

crate::print_values!(
    Farey::new_big(3), 0, 10;
    Farey::new_big(4), 0, 19;
    Farey::new_big_descending(3), 0, 10;
    Farey::new_big_descending(7), 0, 19;
);
