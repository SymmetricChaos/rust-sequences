use crate::{Number, core::traits::Increment, utils::divisibility::aliquot_sum};

/// The aliquot sum of each positive integer, the sum of all of its divisors except itself.
///
/// ```text
/// 0, 1, 1, 3, 1, 6, 1, 7, 4, 8, 1, 16, 1, 10, 9, 15, 1, 21, 1, 22, 11...
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
/// n = 564
/// 564, 780, 1572, 2124, 3336, 5064, 7656, 13944, 26376, 49464, 88536...
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
    AliquotSums::new(), [0, 1, 1, 3, 1, 6, 1, 7, 4, 8, 1, 16, 1, 10, 9, 15, 1, 21, 1, 22, 11, 14, 1, 36, 6, 16, 13, 28, 1, 42, 1, 31, 15, 20, 13, 55, 1, 22, 17, 50, 1, 54, 1, 40, 33, 26, 1, 76, 8, 43, 21, 46, 1, 66, 17, 64, 23, 32, 1, 108, 1, 34, 41, 63, 19, 78, 1, 58, 27, 74, 1, 123, 1, 40, 49, 64, 19, 90, 1, 106];
);

crate::sample_sequences!(
    AliquotSums::new();
    AliquotSequence::new(564);
);
