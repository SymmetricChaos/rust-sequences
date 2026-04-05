use num::{BigInt, CheckedAdd, Integer};

/// The rows of the Bell triangle. The leftmost values are the Bell numbers, which count the number of ways to partition a set with n elements.
pub struct BellTriangle<T> {
    row: Vec<T>,
}

impl<T: Integer + Clone + CheckedAdd> BellTriangle<T> {
    pub fn new() -> Self {
        Self {
            row: vec![T::one()],
        }
    }
}

impl BellTriangle<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Integer + Clone + CheckedAdd> Iterator for BellTriangle<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.row.clone();
        let mut next_row = Vec::with_capacity(self.row.len() + 1);
        next_row.push(self.row.last().unwrap().clone());
        for i in 0..self.row.len() {
            next_row.push(next_row[i].checked_add(&self.row[i])?);
        }
        self.row = next_row;
        Some(out)
    }
}

/// The Bell numbers. The number of ways to partition a set with n elements.
///
/// 1, 1, 2, 5, 15, 52, 203, 877...
pub struct Bell<T> {
    tri: BellTriangle<T>,
}

impl<T: Integer + Clone + CheckedAdd> Bell<T> {
    pub fn new() -> Self {
        Self {
            tri: BellTriangle::new(),
        }
    }
}

impl Bell<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Integer + Clone + CheckedAdd> Iterator for Bell<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.tri.next().unwrap()[0].clone())
    }
}

crate::check_sequences!(
    Bell::new_big(), [1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147];
    BellTriangle::new_big().flatten(), [1, 1, 2, 2, 3, 5, 5, 7, 10, 15];
);
