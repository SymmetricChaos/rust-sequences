use crate::{Number, gray::Gray};

/// For each natural number the number of "runs" of matching bits in the binary expension. The number zero has no bits and is uniquely given a count of zero runs.
///
/// ```text
/// 0, 1, 2, 1, 2, 3, 2, 1, 2, 3, 4, 3...
/// ```
pub struct BinaryRuns<T> {
    gray: Gray<T>,
}

impl BinaryRuns<Number> {
    pub fn new() -> Self {
        Self { gray: Gray::new() }
    }
}

impl Iterator for BinaryRuns<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.gray.next()?.count_ones() as Number)
    }
}

crate::check_sequences!(
    BinaryRuns::new(), [0, 1, 2, 1, 2, 3, 2, 1, 2, 3, 4, 3, 2, 3, 2, 1, 2, 3, 4, 3, 4, 5, 4, 3, 2, 3, 4, 3, 2, 3, 2, 1, 2, 3, 4, 3, 4, 5, 4, 3, 4, 5, 6, 5, 4, 5, 4, 3, 2, 3, 4, 3, 4, 5, 4, 3, 2, 3, 4, 3, 2, 3, 2, 1, 2, 3, 4, 3, 4, 5, 4, 3, 4, 5, 6, 5, 4, 5, 4, 3, 4, 5, 6, 5, 6, 7, 6, 5, 4, 5, 6, 5, 4, 5];
);
