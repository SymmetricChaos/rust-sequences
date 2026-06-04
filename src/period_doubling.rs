use crate::{Number, ruler::Ruler};
use num::{BigInt, CheckedAdd, Integer};

/// The period doubling sequence. Fixed point for the string rewrite rule 0 -> 01 and 1 -> 00. Equivalent to the parity of the 2-adic valuation of each non-negative integer.
///
/// ```text
/// 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0...
/// ```
pub struct PeriodDoubling<T> {
    two_adic_val: Ruler<T>,
    two: T,
}

impl PeriodDoubling<Number> {
    pub fn new() -> Self {
        Self {
            two_adic_val: Ruler::new(),
            two: 2,
        }
    }
}

impl PeriodDoubling<BigInt> {
    pub fn new_big() -> Self {
        Self {
            two_adic_val: Ruler::new_big(),
            two: BigInt::from(2),
        }
    }
}

impl<T: Clone + CheckedAdd + Integer> Iterator for PeriodDoubling<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.two_adic_val.next()?;
        Some(n % self.two.clone())
    }
}

crate::check_iteration_times!(
    PeriodDoubling::new(), 1_000_000;
    PeriodDoubling::new_big(), 1_000_000;
);

crate::check_sequences!(
    PeriodDoubling::new_big(),
    [
        0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1,
        0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0,
        0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1,
        0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0
    ];
);
