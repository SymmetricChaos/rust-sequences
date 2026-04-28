use crate::gray::Gray;
use num::{Integer, PrimInt};

/// For each natural number the number of "runs" of matching bits in the binary expension. The number zero has no bits and is uniquely given a count of zero runs.
///
/// 0, 1, 2, 1, 2, 3, 2, 1, 2, 3, 4, 3...
pub struct BinaryRuns<T> {
    gray: Gray<T>,
}

impl<T: PrimInt + Integer> BinaryRuns<T> {
    pub fn new() -> Self {
        Self { gray: Gray::new() }
    }
}

impl<T: PrimInt + Integer> Iterator for BinaryRuns<T> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.gray.next().map(|n| n.count_ones())
    }
}

crate::check_sequences!(
    BinaryRuns::<u8>::new(), [0, 1, 2, 1, 2, 3, 2, 1, 2, 3, 4, 3, 2, 3, 2, 1, 2, 3, 4, 3, 4, 5, 4, 3, 2, 3, 4, 3, 2, 3, 2, 1, 2, 3, 4, 3, 4, 5, 4, 3, 4, 5, 6, 5, 4, 5, 4, 3, 2, 3, 4, 3, 4, 5, 4, 3, 2, 3, 4, 3, 2, 3, 2, 1, 2, 3, 4, 3, 4, 5, 4, 3, 4, 5, 6, 5, 4, 5, 4, 3, 4, 5, 6, 5, 6, 7, 6, 5, 4, 5, 6, 5, 4, 5];
);
