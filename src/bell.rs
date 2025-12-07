use num::{BigInt, One};

/// The rows of the Bell triangle.
pub struct BellTriangle {
    row: Vec<BigInt>,
}

impl BellTriangle {
    pub fn new_big() -> Self {
        Self {
            row: vec![BigInt::one()],
        }
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

crate::print_values!(
    print_triangles, formatter "{:?}", sep "\n";
    BellTriangle::new_big(), 0, 5;
);

crate::print_values!(
    BellTriangle::new_big().flatten(), 0, 15;
);
