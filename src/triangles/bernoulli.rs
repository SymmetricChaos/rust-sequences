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

crate::print_rows!(
    BernoullisTriangle::new(), 0, 10;
);
