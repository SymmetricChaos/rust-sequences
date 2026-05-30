use crate::{Number, core::traits::Increment, utils::divisibility::aliquot_sum};

/// The aliquot sum of each positive integer, the sum of all of its divisors except itself.
///
/// ```text
/// 0, 1, 1, 3, 1, 6, 1, 7, 4, 8...
/// ```
pub struct AliquotSums {
    ctr: Number,
}

impl AliquotSums {
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for AliquotSums {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.incr()?;
        aliquot_sum(self.ctr)
    }
}

/// The aliquot sequence starting from n. Each term is the aliquot sum of the previous.
///
/// ```text
/// For n = 10:
/// 10, 8, 7, 1, 0, 0, 0...
/// ```
pub struct AliquotSequence {
    n: Number,
}

impl AliquotSequence {
    pub fn new(n: Number) -> Self {
        Self { n }
    }
}

impl Iterator for AliquotSequence {
    type Item = Number;

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
