use num::{BigInt, CheckedAdd, Integer, One, Zero};

use crate::Number;

/// The ruler function. Equivalent to the 2-adic valuation, the exponent of the greatest power of two that divides each positive integer.
///
/// ```text
/// 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, 4...
/// ```
pub struct Ruler<T> {
    p: T,
    ctr: T,
    offset: T,
}

impl Ruler<Number> {
    pub fn new() -> Self {
        Self {
            p: 2,
            ctr: 1,
            offset: 0,
        }
    }

    /// The alternative ruler sequence which removes the zeroes.
    ///
    /// ```text
    /// 1, 2, 1, 3, 1, 2, 1, 4, 1, 2, 1, 3, 1, 2, 1, 5...
    /// ```
    pub fn new_plus() -> Self {
        Self {
            p: 2,
            ctr: 1,
            offset: 1,
        }
    }
}

impl Ruler<BigInt> {
    pub fn new_big() -> Self {
        Self {
            p: BigInt::from(2),
            ctr: BigInt::one(),
            offset: BigInt::zero(),
        }
    }

    /// The alternative ruler sequence which removes the zeroes.
    ///
    /// ```text
    /// 1, 2, 1, 3, 1, 2, 1, 4, 1, 2, 1, 3, 1, 2, 1, 5...
    /// ```
    pub fn new_big_plus() -> Self {
        Self {
            p: BigInt::from(2),
            ctr: BigInt::one(),
            offset: BigInt::one(),
        }
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
    Ruler::new(),      [0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, 4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, 5, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, 4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, 6, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, 4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, 5, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0];
    Ruler::new_plus(), [1, 2, 1, 3, 1, 2, 1, 4, 1, 2, 1, 3, 1, 2, 1, 5, 1, 2, 1, 3, 1, 2, 1, 4, 1, 2, 1, 3, 1, 2, 1, 6, 1, 2, 1, 3, 1, 2, 1, 4, 1, 2, 1, 3, 1, 2, 1, 5, 1, 2, 1, 3, 1, 2, 1, 4, 1, 2, 1, 3, 1, 2, 1, 7, 1, 2, 1, 3, 1, 2, 1, 4, 1, 2, 1, 3, 1, 2, 1, 5, 1, 2, 1, 3, 1, 2, 1, 4, 1, 2, 1, 3, 1, 2, 1, 6, 1, 2, 1, 3, 1, 2, 1, 4, 1];
);
