use num::{BigInt, Integer, One, integer::binomial};

use crate::Number;

/// The Narayana triangle.
///
/// ```text
/// [1], [1,1], [1,3,1], [1,6,6,1]...
/// ```
pub struct NarayanaTriangle<T> {
    row: T,
    idx: T,
}

impl NarayanaTriangle<Number> {
    pub fn new() -> Self {
        Self { row: 1, idx: 1 }
    }
}

#[cfg(feature = "big_int")]
impl NarayanaTriangle<BigInt> {
    pub fn new_big() -> Self {
        Self {
            row: BigInt::one(),
            idx: BigInt::one(),
        }
    }
}

impl<T: Integer + Clone> Iterator for NarayanaTriangle<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut out = Vec::new();
        loop {
            let a0 = binomial(self.row.clone() - T::one(), self.idx.clone() - T::one());
            let a1 = binomial(self.row.clone(), self.idx.clone() - T::one());
            out.push(a0 * a1 / self.idx.clone());
            self.idx = self.idx.clone() + T::one();
            if self.idx > self.row {
                self.row = self.row.clone() + T::one();
                self.idx = T::one();
                break;
            }
        }
        Some(out)
    }
}

crate::print_sequences!(
    NarayanaTriangle::new_big(), 5, "{:?}", "\n";
);
