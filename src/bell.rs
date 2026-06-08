use crate::Number;
use num::{BigInt, One};

/// The rows of the Bell triangle. The leftmost values are the Bell numbers, which count the number of ways to partition a set with n elements.
///
/// ```text
/// [1], ]1,2], [2,3,5], [5,7,10,15]...
/// ```
pub struct BellTriangle<T> {
    row: Vec<T>,
}

impl BellTriangle<Number> {
    pub fn new() -> Self {
        Self { row: vec![1] }
    }
}

#[cfg(feature = "big_int")]
impl BellTriangle<BigInt> {
    pub fn new_big() -> Self {
        Self {
            row: vec![BigInt::one()],
        }
    }
}

impl Iterator for BellTriangle<Number> {
    type Item = Vec<Number>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.row.clone();
        let mut next_row = Vec::with_capacity(self.row.len() + 1);
        next_row.push(self.row.last().unwrap().clone());
        for i in 0..self.row.len() {
            next_row.push(next_row[i].checked_add(self.row[i])?);
        }
        self.row = next_row;
        Some(out)
    }
}

#[cfg(feature = "big_int")]
impl Iterator for BellTriangle<BigInt> {
    type Item = Vec<BigInt>;

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
/// ```text
/// 1, 1, 2, 5, 15, 52, 203, 877...
/// ```
pub struct Bell<T> {
    tri: BellTriangle<T>,
}

impl Bell<Number> {
    pub fn new() -> Self {
        Self {
            tri: BellTriangle::new(),
        }
    }
}

#[cfg(feature = "big_int")]
impl Bell<BigInt> {
    pub fn new_big() -> Self {
        Self {
            tri: BellTriangle::new_big(),
        }
    }
}

impl Iterator for Bell<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.tri.next().unwrap()[0])
    }
}

#[cfg(feature = "big_int")]
impl Iterator for Bell<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.tri.next().unwrap()[0].clone())
    }
}

crate::check_sequences!(
    Bell::new_big(), [1_u64, 1, 2, 5, 15, 52, 203, 877, 4140, 21147, 115975, 678570, 4213597, 27644437, 190899322, 1382958545, 10480142147, 82864869804, 682076806159, 5832742205057, 51724158235372, 474869816156751, 4506715738447323, 44152005855084346, 445958869294805289, 4638590332229999353];
    BellTriangle::new_big().flatten(), [1, 1, 2, 2, 3, 5, 5, 7, 10, 15];
);
