use num::{BigInt, CheckedAdd, CheckedMul, One, Zero};

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
pub struct UnsignedStirlingFirst<T> {
    n: T,
    row: Vec<T>,
}

impl<T: One + Zero + CheckedAdd + CheckedMul + Clone> UnsignedStirlingFirst<T> {
    pub fn new() -> Self {
        Self {
            n: T::zero(),
            row: vec![T::one()],
        }
    }
}

impl UnsignedStirlingFirst<BigInt> {
    pub fn new_big() -> Self {
        Self {
            n: BigInt::zero(),
            row: vec![BigInt::one()],
        }
    }
}

impl<T: One + Zero + CheckedAdd + CheckedMul + Clone> Iterator for UnsignedStirlingFirst<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = Some(self.row.clone());
        let mut next_row = Vec::with_capacity(self.row.len() + 1);
        next_row.push(T::zero()); // every row except the first starts with zero

        for k in 1..self.row.len() {
            next_row.push(
                self.n
                    .checked_mul(&self.row[k])?
                    .checked_add(&self.row[k - 1])?,
            );
        }
        next_row.push(T::one()); // every row ends with one

        self.n = self.n.checked_add(&T::one())?;
        self.row = next_row;

        out
    }
}

pub struct StirlingSecond {}

check_sequences!(
    UnsignedStirlingFirst::new_big().flatten(), 0, 20, [1, 0, 1, 0, 1, 1, 0, 2, 3, 1, 0, 6, 11, 6, 1, 0, 24, 50, 35, 10];
);
