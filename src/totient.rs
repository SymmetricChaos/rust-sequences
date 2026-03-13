use crate::utils::divisibility::cototient;

/// The totient of each positive integer. For each positive integer n, the number of positive integers less than n which are coprime to n.
/// 1, 1, 2, 2, 4, 2, 6, 4, 6, 4...
pub struct Totients {
    ctr: u64,
}

impl Totients {
    /// Only u64 output is supported.
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for Totients {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr = self.ctr.checked_add(1)?;
        Some(crate::utils::divisibility::totient(self.ctr))
    }
}

/// The cototient of each positive integer. Each positive integer n, minus the number of positive integers less than n which are coprime to n.
/// 0, 1, 1, 2, 1, 4, 1, 4, 3, 6...
pub struct Cototients {
    ctr: u64,
}

impl Cototients {
    /// Only u64 output is supported.
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for Cototients {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr = self.ctr.checked_add(1)?;
        Some(cototient(self.ctr))
    }
}

crate::check_sequences!(
    Totients::new(), [1, 1, 2, 2, 4, 2, 6, 4, 6, 4];
    Cototients::new(), [0, 1, 1, 2, 1, 4, 1, 4, 3, 6];
);
