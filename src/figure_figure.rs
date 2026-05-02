use num::{CheckedAdd, Integer};
use std::collections::VecDeque;

/// The Figure-Figure sequence introduced by Hoftsader. Along with its first differences includes every positive integer exactly once. This property is not unique but this sequence is the lexicographically first.
///
/// 1, 3, 7, 12, 18, 26, 35, 45, 56, 69...
pub struct FigureFigure<T> {
    ctr: T,
    comp: T,
    terms: VecDeque<T>,
}

impl<T: CheckedAdd + Clone + Integer> FigureFigure<T> {
    pub fn new() -> Self {
        Self {
            ctr: T::one(),
            comp: T::one() + T::one(),
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
    FigureFigure::<i32>::new(), [1, 3, 7, 12, 18, 26, 35, 45, 56, 69, 83, 98, 114, 131, 150, 170, 191, 213, 236];
);
