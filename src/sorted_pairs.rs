use num::{BigInt, CheckedAdd, One, Zero};

/// All pairs of non-negative integers where the first is not more than the second.
pub struct SortedPairs<T> {
    row: T,
    col: T,
}

impl<T: CheckedAdd + Clone + One + PartialOrd + Zero> SortedPairs<T> {
    pub fn new() -> Self {
        Self {
            row: T::zero(),
            col: T::zero(),
        }
    }
}

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

        self.col = self.col.checked_add(&T::one())?;
        if self.col > self.row {
            self.col = T::zero();
            self.row = self.row.checked_add(&T::one())?;
        }

        out
    }
}

/// All pairs of non-negative integers where the first is less than the second.
pub struct SortedPairsStrict<T> {
    row: T,
    col: T,
}

impl<T: CheckedAdd + Clone + One + PartialOrd + Zero> SortedPairsStrict<T> {
    pub fn new() -> Self {
        Self {
            row: T::one(),
            col: T::zero(),
        }
    }
}

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

        self.col = self.col.checked_add(&T::one())?;
        if self.col >= self.row {
            self.col = T::zero();
            self.row = self.row.checked_add(&T::one())?;
        }

        out
    }
}

crate::print_sequences!(
    pairs, formatter "{:?}", sep ", ";
    SortedPairsStrict::<i32>::new(), 0, 10;
    SortedPairs::<i32>::new(), 0, 10;
);
