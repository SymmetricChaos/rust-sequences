use std::ops::Sub;

use num::BigInt;

/// A Ducci sequence.
/// Each term a tuple containing the absolute difference of each adjacent pair of the previous tuple (with the last adjacent to the first).
/// Ducci sequences with a number of elements a that is a power of two eventually reach an all zero state.
/// Ducci sequences with other numbers of elements eventually become periodic and consist of only 0s and a single positive integer.
pub struct Ducci<T> {
    tup: Vec<T>,
}

impl<T: Clone + Ord + Sub<Output = T>> Ducci<T> {
    pub fn new(tup: Vec<T>) -> Self {
        Self { tup }
    }
}

impl Ducci<BigInt> {
    pub fn new_big(tup: Vec<BigInt>) -> Self {
        Self { tup }
    }
}

impl<T: Clone + Ord + Sub<Output = T>> Iterator for Ducci<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.tup.clone();
        for i in 0..self.tup.len() {
            let a = out[i].clone();
            let b = out[(i + 1) % self.tup.len()].clone();
            if a >= b {
                self.tup[i] = a - b;
            } else {
                self.tup[i] = b - a;
            }
        }
        Some(out)
    }
}

crate::print_sequences!(
    Ducci::new(vec![3,7,28,1]), 0, 7, "{:?}", "\n";
    Ducci::new(vec![2,5,2,5,2]), 0, 10, "{:?}", "\n";
);
