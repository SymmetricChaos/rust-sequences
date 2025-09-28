use num::{BigInt, CheckedAdd, CheckedMul, Integer, PrimInt, Zero, rational::Ratio};

use crate::core::Factorials;

/// Convergents of e as defined in the usual ways as the series of reciprocals of factorials.
/// 0, 1, 2, 5/2, 8/3, 65/24, 163/60...
pub struct Euler<T> {
    factorials: Factorials<T>,
    sum: Ratio<T>,
}

impl<T: PrimInt + Integer> Euler<T> {
    pub fn new() -> Self {
        Self {
            factorials: Factorials::<T>::new(),
            sum: Ratio::<T>::zero(),
        }
    }
}

impl Euler<BigInt> {
    pub fn new_big() -> Self {
        Self {
            factorials: Factorials::new_big(),
            sum: Ratio::<BigInt>::zero(),
        }
    }
}

impl<T: Clone + Integer + CheckedMul + CheckedAdd> Iterator for Euler<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.sum.clone();
        let rat = Ratio::new_raw(T::one(), self.factorials.next()?); // the ratio is always fully reduced
        self.sum = self.sum.checked_add(&rat)?;
        Some(out)
    }
}

#[cfg(test)]
use crate::core::rational_digits::DecimalDigits;
#[cfg(test)]
use itertools::Itertools;

crate::print_values!(
    Euler::<u64>::new(), 0, 10;
    Euler::new_big().map(|x| DecimalDigits::from_ratio_big(x).map(|d| d.to_string()).take(6).join("")), 0, 10;
);
