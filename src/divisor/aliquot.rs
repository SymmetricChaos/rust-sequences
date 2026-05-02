use crate::utils::divisibility::aliquot_sum;

/// The aliquot sum of each positive integer, the sum of all of its divisors except itself.
///
/// 0, 1, 1, 3, 1, 6, 1, 7, 4, 8...
pub struct AliquotSums {
    ctr: u64,
}

impl AliquotSums {
    /// Only u64 output is supported.
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for AliquotSums {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr = self.ctr.checked_add(1)?;
        aliquot_sum(self.ctr)
    }
}

/// The aliquot sequence starting from n. Each term is the aliquot sum of the previous.
pub struct AliquotSequence {
    n: u64,
}

impl AliquotSequence {
    /// Only u64 output is supported.
    pub fn new(n: u64) -> Self {
        Self { n }
    }
}

impl Iterator for AliquotSequence {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.n;
        self.n = aliquot_sum(self.n)?;
        Some(out)
    }
}
crate::check_sequences!(
    AliquotSums::new(), [0, 1, 1, 3, 1, 6, 1, 7, 4, 8];
    AliquotSequence::new(10), [10, 8, 7, 1, 0, 0, 0, 0, 0, 0];
);
