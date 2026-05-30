use crate::{
    Number,
    core::traits::Increment,
    utils::totient::{cototient, totient},
};

/// The totient of each positive integer. For each positive integer n, the number of positive integers less than n which are coprime to n.
///
/// ```text
/// 1, 1, 2, 2, 4, 2, 6, 4, 6, 4...
/// ```
pub struct Totients {
    ctr: Number,
}

impl Totients {
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for Totients {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.incr()?;
        Some(totient(self.ctr))
    }
}

/// The cototient of each positive integer. Each positive integer n, minus the number of positive integers less than n which are coprime to n.
///
/// ```text
/// 0, 1, 1, 2, 1, 4, 1, 4, 3, 6...
/// ```
pub struct Cototients {
    ctr: Number,
}

impl Cototients {
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for Cototients {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.incr()?;
        Some(cototient(self.ctr))
    }
}

crate::check_sequences!(
    Totients::new(),   [1, 1, 2, 2, 4, 2, 6, 4, 6, 4, 10, 4, 12, 6, 8, 8, 16, 6, 18, 8, 12, 10, 22, 8, 20, 12, 18, 12, 28, 8, 30, 16, 20, 16, 24, 12, 36, 18, 24, 16, 40, 12, 42, 20, 24, 22, 46, 16, 42, 20, 32, 24, 52, 18, 40, 24, 36, 28, 58, 16, 60, 30, 36, 32, 48, 20, 66, 32, 44];
    Cototients::new(), [0, 1, 1, 2, 1, 4, 1, 4, 3, 6, 1, 8, 1, 8, 7, 8, 1, 12, 1, 12, 9, 12, 1, 16, 5, 14, 9, 16, 1, 22, 1, 16, 13, 18, 11, 24, 1, 20, 15, 24, 1, 30, 1, 24, 21, 24, 1, 32, 7, 30, 19, 28, 1, 36, 15, 32, 21, 30, 1, 44, 1, 32, 27, 32, 17, 46, 1, 36, 25, 46, 1, 48, 1, 38, 35, 40, 17, 54, 1, 48, 27];
);
