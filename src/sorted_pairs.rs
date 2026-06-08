use crate::{Number, core::traits::Increment};
use num::{BigInt, CheckedAdd, One, Zero};

/// All pairs of non-negative integers where the first is less than or equal to the second.
pub struct SortedPairs<T> {
    row: T,
    col: T,
}

impl SortedPairs<Number> {
    pub fn new() -> Self {
        Self { row: 0, col: 0 }
    }
}

#[cfg(feature = "big_int")]
impl SortedPairs<BigInt> {
    pub fn new_big() -> Self {
        Self {
            row: BigInt::zero(),
            col: BigInt::zero(),
        }
    }
}

impl<T: CheckedAdd + Clone + One + PartialOrd + Zero> Iterator for SortedPairs<T> {
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let out = Some((self.col.clone(), self.row.clone()));

        self.col.incr()?;
        if self.col > self.row {
            self.col = T::zero();
            self.row.incr()?;
        }

        out
    }
}

/// All pairs of non-negative integers where the first is less than the second.
pub struct SortedPairsStrict<T> {
    row: T,
    col: T,
}

impl SortedPairsStrict<Number> {
    pub fn new() -> Self {
        Self { row: 1, col: 0 }
    }
}

#[cfg(feature = "big_int")]
impl SortedPairsStrict<BigInt> {
    pub fn new_big() -> Self {
        Self {
            row: BigInt::one(),
            col: BigInt::zero(),
        }
    }
}

impl<T: CheckedAdd + Clone + One + PartialOrd + Zero> Iterator for SortedPairsStrict<T> {
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let out = Some((self.col.clone(), self.row.clone()));

        self.col.incr()?;
        if self.col >= self.row {
            self.col = T::zero();
            self.row.incr()?;
        }

        out
    }
}

crate::print_sequences!(
    SortedPairsStrict::new(), 10, "{:?}", ", ";
    SortedPairs::new(), 10, "{:?}", ", ";
);
