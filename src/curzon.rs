use crate::{Number, core::traits::Increment, utils::exp_by_squaring::checked_pow_mod};
use num::{BigInt, FromPrimitive, Zero};

/// The Curzon numbers. Those for which k^n+1 is a multiple of k*n+1 for positive integers n. Only even values of k have solutions.
///
/// ```text
/// For k = 2
/// 1, 2, 5, 6, 9, 14, 18, 21, 26, 29, 30, 33, 41...
/// ```
pub struct Curzon<T> {
    base: T,
    ctr: T,
}

impl Curzon<Number> {
    /// Base must be greater than or equal to 2.
    pub fn new(base: Number) -> Self {
        assert!(base >= 2);
        Self { base, ctr: 0 }
    }
}

impl Curzon<BigInt> {
    pub fn new_big<G>(base: G) -> Self
    where
        BigInt: From<G>,
    {
        let base = BigInt::from(base);
        assert!(base >= BigInt::from_i32(2).unwrap());
        Self {
            base,
            ctr: BigInt::zero(),
        }
    }
}

impl Iterator for Curzon<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.ctr.incr()?;
            let modulus = self.ctr.checked_mul(self.base)?.checked_add(1)?;
            let p = checked_pow_mod(self.base.clone(), self.ctr.clone(), modulus.clone())?;
            if p == modulus - 1 {
                return Some(self.ctr.clone());
            }
        }
    }
}

impl Iterator for Curzon<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.ctr.incr()?;
            let modulus: BigInt = (&self.ctr * &self.base) + 1;
            let p = checked_pow_mod(self.base.clone(), self.ctr.clone(), modulus.clone())?;
            if p == modulus - 1 {
                return Some(self.ctr.clone());
            }
        }
    }
}

crate::check_sequences!(
    Curzon::new(2), [1, 2, 5, 6, 9, 14, 18, 21, 26, 29, 30, 33, 41, 50, 53, 54, 65, 69, 74, 78, 81, 86, 89, 90, 98, 105, 113, 114, 125, 134, 138, 141, 146, 153, 158, 165, 173, 174, 186, 189, 194, 198, 209, 210, 221, 230, 233, 245, 249, 254, 261, 270, 273, 278, 281, 285, 293];
);
