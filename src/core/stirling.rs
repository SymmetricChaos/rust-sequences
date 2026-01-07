use num::{BigInt, One, Zero};

use crate::check_sequences;

pub fn unsigned_stirling_first(n: &BigInt, k: &BigInt) -> BigInt {
    if n == k {
        return BigInt::one();
    }
    if k.is_zero() || n.is_zero() {
        return BigInt::zero();
    }

    (n - 1) * unsigned_stirling_first(&(n - 1), &k) + unsigned_stirling_first(&(n - 1), &(k - 1))
}

/// Unsigned Stirling numbers of the first kind by rows.
pub struct UnsignedStirlingFirst {
    n: BigInt,
    row: Vec<BigInt>,
}

impl UnsignedStirlingFirst {
    pub fn new() -> Self {
        Self {
            n: BigInt::zero(),
            row: vec![BigInt::one()],
        }
    }
}

impl Iterator for UnsignedStirlingFirst {
    type Item = Vec<BigInt>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = Some(self.row.clone());
        let mut next_row = Vec::with_capacity(self.row.len() + 1);
        next_row.push(BigInt::zero()); // every row except the first starts with zero

        for k in 1..self.row.len() {
            next_row.push(&self.n * &self.row[k] + &self.row[k - 1]);
        }
        next_row.push(BigInt::one()); // every row ends with one

        self.n += 1;
        self.row = next_row;

        out
    }
}

pub struct StirlingSecond {}

check_sequences!(
    UnsignedStirlingFirst::new().flatten(), 0, 20, [1, 0, 1, 0, 1, 1, 0, 2, 3, 1, 0, 6, 11, 6, 1, 0, 24, 50, 35, 10];
);
