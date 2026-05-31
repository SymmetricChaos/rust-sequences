use crate::Number;
use num::BigInt;

/// A Ducci sequence.
/// Each term a tuple containing the absolute difference of each adjacent pair of the previous tuple (with the last adjacent to the first).
/// Ducci sequences with a number of elements a that is a power of two eventually reach an all zero state.
/// Ducci sequences with other numbers of elements eventually become periodic and consist of only 0s and a single positive integer.
pub struct Ducci<T> {
    tup: Vec<T>,
}

impl Ducci<Number> {
    pub fn new(tup: Vec<Number>) -> Self {
        Self { tup }
    }
}

impl Ducci<BigInt> {
    pub fn new_big<G>(tup: Vec<G>) -> Self
    where
        BigInt: From<G>,
    {
        let tup = tup.into_iter().map(|g| BigInt::from(g)).collect();
        Self { tup }
    }
}

impl Iterator for Ducci<Number> {
    type Item = Vec<Number>;

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

impl Iterator for Ducci<BigInt> {
    type Item = Vec<BigInt>;

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
    Ducci::new(vec![3,7,28,1]), 7, "{:?}", "\n";
    Ducci::new(vec![2,5,2,5,2]), 10, "{:?}", "\n";
);
