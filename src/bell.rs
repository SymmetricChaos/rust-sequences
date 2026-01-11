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

/// The Bell numbers. The number of ways to partition a set with n elements.
pub struct Bell {
    tri: BellTriangle,
}

impl Bell {
    pub fn new_big() -> Self {
        Self {
            tri: BellTriangle::new_big(),
        }
    }
}

impl Iterator for Bell {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.tri.next().unwrap()[0].clone())
    }
}

crate::check_sequences!(
    Bell::new_big(), 0, 10, [1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147];
    BellTriangle::new_big().flatten(), 0, 10, [1, 1, 2, 2, 3, 5, 5, 7, 10, 15];
);
