use crate::core::nth_powers::NthPowers;
use num::{BigInt, CheckedAdd, CheckedMul, Integer, One, Zero, rational::Ratio};
use std::iter::Skip;

/// The partial sums of the Riemann zeta function for natural numbers greater than zero
pub struct Zeta<T> {
    sum: Ratio<T>,
    powers: Skip<NthPowers<T>>,
}

impl<T: CheckedAdd + CheckedMul + Clone + One + Integer> Zeta<T> {
    /// Power p is specified as a u32 due to the interface of the .pow() function.
    pub fn new(p: u32) -> Self {
        Self {
            sum: Ratio::zero(),
            powers: NthPowers::new(p).skip(1),
        }
    }
}

impl Zeta<BigInt> {
    /// Power p is specified as a u32 due to the interface of the .pow() function.
    pub fn new_big(p: u32) -> Self {
        Self {
            sum: Ratio::zero(),
            powers: NthPowers::new_big(p).skip(1),
        }
    }
}

impl<T: CheckedAdd + CheckedMul + Clone + One + Integer> Iterator for Zeta<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.sum.clone();

        let term = Ratio::new_raw(T::one(), self.powers.next()?);
        self.sum = self.sum.checked_add(&term)?;

        Some(out)
    }
}

// very slow to converge
crate::check_sequences!(
    crate::core::rational_digits::DecimalDigits::from_ratio_big(Zeta::new_big(2).skip(1500).next().unwrap(), 10.into()), 0, 4, [1, 6, 4, 4]; // pi squared over 6 (1.644...)
);
