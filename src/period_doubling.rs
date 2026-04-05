use crate::padic_valuation::PadicValuation;
use num::{BigInt, CheckedAdd, Integer};
use std::ops::Rem;

/// The period doubling sequence.
///
/// 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0...
pub struct PeriodDoubling<T> {
    two_adic_val: PadicValuation<T>,
}

impl<T: Clone + CheckedAdd + Integer> PeriodDoubling<T> {
    pub fn new() -> Self {
        Self {
            two_adic_val: PadicValuation::new(T::one() + T::one()),
        }
    }
}

impl PeriodDoubling<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Clone + CheckedAdd + Integer + Rem> Iterator for PeriodDoubling<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.two_adic_val.next()?;
        Some(n % (T::one() + T::one()))
    }
}

crate::check_sequences!(
    PeriodDoubling::new_big(),
    [
        0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1,
        0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0,
        0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1,
        0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0
    ];
);
