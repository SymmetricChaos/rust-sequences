use num::{BigInt, CheckedAdd, CheckedMul, Integer, Zero, rational::Ratio};

use crate::core::factorial::Factorials;

/// Convergents of e as the series of partial sums of the reciprocals of factorials.
/// 0, 1, 2, 5/2, 8/3, 65/24, 163/60...
pub struct Euler<T> {
    factorials: Factorials<T>,
    sum: Ratio<T>,
}

impl<T: CheckedAdd + CheckedMul + Clone + Integer> Euler<T> {
    pub fn new() -> Self {
        Self {
            factorials: Factorials::new(),
            sum: Ratio::zero(),
        }
    }
}

impl Euler<BigInt> {
    pub fn new_big() -> Self {
        Self {
            factorials: Factorials::new_big(),
            sum: Ratio::zero(),
        }
    }
}

impl<T: CheckedAdd + CheckedMul + Clone + Integer> Iterator for Euler<T> {
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
    Euler::new_big().map(|x| DecimalDigits::from_ratio_big(x, BigInt::from(10)).map(|d| d.to_string()).take(6).join("")), 0, 10;
);
