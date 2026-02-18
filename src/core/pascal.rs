use num::{BigInt, CheckedAdd, One};

/// The rows of Pascal's triangle, aka the binomial coefficients.
pub struct PascalsTriangle<T> {
    row: Vec<T>,
}

impl<T: CheckedAdd + Clone + One> PascalsTriangle<T> {
    pub fn new() -> Self {
        Self {
            row: vec![T::one()],
        }
    }
}

impl PascalsTriangle<BigInt> {
    pub fn new_big() -> Self {
        Self {
            row: vec![BigInt::one()],
        }
    }
}

impl<T: CheckedAdd + Clone + One> Iterator for PascalsTriangle<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.row.clone();
        let mut next_row = Vec::with_capacity(self.row.len() + 1);
        next_row.push(T::one());
        for i in 0..(self.row.len() - 1) {
            next_row.push(self.row[i].checked_add(&self.row[i + 1])?);
        }
        next_row.push(T::one());
        self.row = next_row;
        Some(out)
    }
}

/// Bernoulli's triangle. The partial sums of the rows of Pascal's triangle.
pub struct BernoullisTriangle<T> {
    row: Vec<T>,
}

impl<T: CheckedAdd + Clone + One> BernoullisTriangle<T> {
    pub fn new() -> Self {
        Self {
            row: vec![T::one()],
        }
    }

    /// Return an iterator over the elements in each row.
    pub fn as_ints() -> impl Iterator<Item = T> {
        Self::new().flatten()
    }
}

impl BernoullisTriangle<BigInt> {
    pub fn new_big() -> Self {
        Self {
            row: vec![BigInt::one()],
        }
    }

    /// Return an iterator over the elements in each row.
    pub fn as_ints_big() -> impl Iterator<Item = BigInt> {
        Self::new_big().flatten()
    }
}

impl<T: CheckedAdd + Clone + One> Iterator for BernoullisTriangle<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.row.clone();
        let mut next_row = Vec::with_capacity(self.row.len() + 1);
        next_row.push(T::one());
        for i in 0..(self.row.len() - 1) {
            next_row.push(self.row[i].checked_add(&self.row[i + 1])?);
        }
        next_row.push(self.row.last()?.checked_add(self.row.last()?)?);
        self.row = next_row;
        Some(out)
    }
}

crate::print_values!(
    print_triangles, formatter "{:?}", sep "\n";
    PascalsTriangle::new_big(), 0, 5;
    BernoullisTriangle::new_big(), 0, 5;
);

crate::check_sequences!(
    PascalsTriangle::<i32>::new().flatten(), [1, 1, 1, 1, 2, 1, 1, 3, 3, 1, 1, 4, 6, 4, 1, 1, 5, 10, 10, 5];
    BernoullisTriangle::<i32>::new().flatten(), [1, 1, 2, 1, 3, 4, 1, 4, 7, 8, 1, 5, 11, 15, 16, 1, 6, 16, 26, 31];
);
