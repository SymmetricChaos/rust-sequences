use num::{BigInt, One};

/// The partial sums of the rows of Pascal's triangle.
pub struct BernoullisTriangle {
    row: Vec<BigInt>,
}

impl BernoullisTriangle {
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

impl Iterator for BernoullisTriangle {
    type Item = Vec<BigInt>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.row.clone();
        let mut next_row = Vec::with_capacity(self.row.len() + 1);
        next_row.push(BigInt::one());
        for i in 0..(self.row.len() - 1) {
            next_row.push(&self.row[i] + &self.row[i + 1]);
        }
        next_row.push(self.row.last().unwrap() * 2);
        self.row = next_row;
        Some(out)
    }
}
