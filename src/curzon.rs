use crate::{core::traits::Increment, utils::exp_by_squaring::checked_pow_mod};
use num::{CheckedAdd, CheckedMul, Integer};

/// The Curzon numbers. Those for which k^n+1 is a multiple of k*n+1 for positive integers n. Only even values of k have solutions.
///
/// For k = 2
///
/// 1, 2, 5, 6, 9, 14, 18, 21, 26, 29, 30, 33, 41...
pub struct Curzon<T> {
    base: T,
    ctr: T,
}

impl<T: CheckedMul + Clone + Integer> Curzon<T> {
    pub fn new(base: T) -> Self {
        Self {
            base,
            ctr: T::zero(),
        }
    }
}

impl<T: CheckedAdd + CheckedMul + Clone + Integer> Iterator for Curzon<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.ctr.incr()?;
            let modulus = self.ctr.checked_mul(&self.base)?.checked_add(&T::one())?;
            let p = checked_pow_mod(self.base.clone(), self.ctr.clone(), modulus.clone())?;
            if p == modulus - T::one() {
                return Some(self.ctr.clone());
            }
        }
    }
}

crate::check_sequences!(
    Curzon::new(2), [1, 2, 5, 6, 9, 14, 18, 21, 26, 29, 30, 33, 41, 50, 53, 54, 65, 69, 74, 78, 81, 86, 89, 90, 98, 105, 113, 114, 125, 134, 138, 141, 146, 153, 158, 165, 173, 174, 186, 189, 194, 198, 209, 210, 221, 230, 233, 245, 249, 254, 261, 270, 273, 278, 281, 285, 293];
);
