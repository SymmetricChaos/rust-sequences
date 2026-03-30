use num::{BigInt, CheckedAdd, Integer};

/// The ruler function. Equivalent to the 2-adic valuation, the exponent of the greatest power of two that divides each positive integer.
///
/// 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, 4...
pub struct Ruler<T> {
    p: T,
    ctr: T,
    offset: T,
}

impl<T: CheckedAdd + Clone + Integer> Ruler<T> {
    pub fn new() -> Self {
        Self {
            p: T::one() + T::one(),
            ctr: T::one(),
            offset: T::zero(),
        }
    }

    /// The alternative ruler sequence which removes the zeroes.
    ///
    /// 1, 2, 1, 3, 1, 2, 1, 4, 1, 2, 1, 3, 1, 2, 1, 5...
    pub fn new_plus() -> Self {
        Self {
            p: T::one() + T::one(),
            ctr: T::one(),
            offset: T::one(),
        }
    }
}

impl Ruler<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }

    /// The alternative ruler sequence which removes the zeroes.
    /// 1, 2, 1, 3, 1, 2, 1, 4, 1, 2, 1, 3, 1, 2, 1, 5...
    pub fn new_big_plus() -> Self {
        Self::new_plus()
    }
}

impl<T: CheckedAdd + Clone + Integer> Iterator for Ruler<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut val = self.offset.clone();
        let mut n = self.ctr.clone();
        loop {
            let (q, r) = n.div_rem(&self.p);
            n = q;
            if r.is_zero() {
                val = val + T::one();
            } else {
                break;
            }
        }

        self.ctr = self.ctr.checked_add(&T::one())?;
        Some(val)
    }
}

crate::check_sequences!(
    Ruler::<i32>::new(), [0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, 4, 0, 1, 0, 2];
    Ruler::<i32>::new_plus(), [1, 2, 1, 3, 1, 2, 1, 4, 1, 2, 1, 3, 1, 2, 1, 5];
);
