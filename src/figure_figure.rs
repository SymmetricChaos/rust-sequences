use num::{BigInt, CheckedAdd, Integer, One};
use std::collections::VecDeque;

use crate::Number;

/// The Figure-Figure sequence introduced by Hoftsader. Along with its first differences includes every positive integer exactly once. This property is not unique but this sequence is the lexicographically first.
///
/// ```text
/// 1, 3, 7, 12, 18, 26, 35, 45, 56, 69, 83, 98, 114, 131, 150, 170...
/// ```
pub struct FigureFigure<T> {
    ctr: T,
    comp: T,
    terms: VecDeque<T>,
}

impl FigureFigure<Number> {
    pub fn new() -> Self {
        Self {
            ctr: 1,
            comp: 2,
            terms: VecDeque::new(),
        }
    }
}

#[cfg(feature = "big_int")]
impl FigureFigure<BigInt> {
    pub fn new_big() -> Self {
        Self {
            ctr: BigInt::one(),
            comp: BigInt::from(2),
            terms: VecDeque::new(),
        }
    }
}

impl<T: CheckedAdd + Clone + Integer> Iterator for FigureFigure<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.ctr.clone();

        self.ctr = self.ctr.checked_add(&self.comp)?;
        self.terms.push_back(self.ctr.clone());

        self.comp = self.comp.checked_add(&T::one())?;
        if self.terms[0] <= self.comp {
            self.terms.pop_front();
            self.comp = self.comp.checked_add(&T::one())?;
        }

        Some(out)
    }
}

crate::check_sequences!(
    FigureFigure::new(), [1, 3, 7, 12, 18, 26, 35, 45, 56, 69, 83, 98, 114, 131, 150, 170, 191, 213, 236, 260, 285, 312, 340, 369, 399, 430, 462, 495, 529, 565, 602, 640, 679, 719, 760, 802, 845, 889, 935, 982, 1030, 1079, 1129, 1180, 1232, 1285, 1339, 1394, 1451, 1509, 1568, 1628, 1689];
);

crate::sample_sequences!(
    FigureFigure::new();
);
