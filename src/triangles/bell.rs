use num::{BigInt, One};

/// The rows of the Bell triangle.
pub struct BellTriangle {
    row: Vec<BigInt>,
}

impl BellTriangle {
    pub fn new() -> Self {
        Self {
            row: vec![BigInt::one()],
        }
    }

    /// Return an iterator over the elements in each row.
    pub fn as_ints() -> impl Iterator<Item = BigInt> {
        Self::new().flatten()
    }
}

impl Iterator for BellTriangle {
    type Item = Vec<BigInt>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.row.clone();
        let mut next_row = Vec::with_capacity(self.row.len() + 1);
        next_row.push(self.row.last().unwrap().clone());
        for i in 0..self.row.len() {
            next_row.push(&next_row[i] + &self.row[i]);
        }
        self.row = next_row;
        Some(out)
    }
}
