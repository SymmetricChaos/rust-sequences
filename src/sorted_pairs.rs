use num::{BigInt, CheckedAdd, One, Zero};

pub struct SortedPairs<T> {
    row: T,
    col: T,
}

impl<T: Zero + One> SortedPairs<T> {
    pub fn new() -> Self {
        Self {
            row: T::one(),
            col: T::zero(),
        }
    }
}

impl SortedPairs<BigInt> {
    pub fn new_big() -> Self {
        Self {
            row: BigInt::one(),
            col: BigInt::zero(),
        }
    }
}

impl<T: CheckedAdd + Clone + One + PartialOrd + Zero> Iterator for SortedPairs<T> {
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

crate::print_values!(
    pairs, formatter "{:?}", sep ", ";
    SortedPairs::<i32>::new(), 0, 10;
);
