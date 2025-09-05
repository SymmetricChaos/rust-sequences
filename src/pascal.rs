use num::{BigInt, One};

/// The rows of Pascal's triangle, aka the binomial coefficients.
pub struct PascalsTriangle {
    row: Vec<BigInt>,
}

impl PascalsTriangle {
    pub fn new() -> Self {
        Self {
            row: vec![BigInt::one()],
        }
    }
}

impl Iterator for PascalsTriangle {
    type Item = Vec<BigInt>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.row.clone();
        let mut next_row = Vec::with_capacity(self.row.len());
        next_row.push(BigInt::one());
        for i in 0..(self.row.len() - 1) {
            next_row.push(&self.row[i] + &self.row[i + 1]);
        }
        next_row.push(BigInt::one());
        self.row = next_row;
        Some(out)
    }
}

crate::print_rows!(
    PascalsTriangle::new(), 0, 10;
);
