use crate::{
    Number,
    core::traits::{CountBits, Increment},
};

/// The binary weight of each natural number, starting from zero.
///
/// ```text
/// 0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2...
/// ```
pub struct BinaryWeight<T> {
    ctr: T,
}

impl BinaryWeight<Number> {
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for BinaryWeight<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let n = Some(self.ctr.count_ones_64() as Number);
        self.ctr.incr()?;
        n
    }
}

crate::check_sequences!(
    BinaryWeight::new(), [0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4, 1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 2, 3, 3, 4, 3, 4, 4, 5, 3];
);
