use num::{BigInt, CheckedAdd, One};

use crate::Number;

/// The rows of Pascal's triangle, aka the binomial coefficients.
///
/// ```text
/// [1], [1,1], [1,2,1], [1,3,3,1], [1,4,6,4,1]...
/// ```
pub struct PascalsTriangle<T> {
    row: Vec<T>,
}

impl PascalsTriangle<Number> {
    pub fn new() -> Self {
        Self { row: vec![1] }
    }
}

#[cfg(feature = "big_int")]
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
///
/// [1], [1,2], [1,3,4], [1,4,7,8], [1,5,11,15,16]...
pub struct BernoullisTriangle<T> {
    row: Vec<T>,
}

impl BernoullisTriangle<Number> {
    pub fn new() -> Self {
        Self { row: vec![1] }
    }
}

#[cfg(feature = "big_int")]
impl BernoullisTriangle<BigInt> {
    pub fn new_big() -> Self {
        Self {
            row: vec![BigInt::one()],
        }
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

crate::print_sequences!(
    PascalsTriangle::new_big(), 5, "{:?}", "\n";
    BernoullisTriangle::new_big(), 5, "{:?}", "\n";
);

crate::check_sequences!(
    PascalsTriangle::new().flatten(), [1, 1, 1, 1, 2, 1, 1, 3, 3, 1, 1, 4, 6, 4, 1, 1, 5, 10, 10, 5];
    BernoullisTriangle::new().flatten(), [1, 1, 2, 1, 3, 4, 1, 4, 7, 8, 1, 5, 11, 15, 16, 1, 6, 16, 26, 31];
);
