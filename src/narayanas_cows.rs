use crate::Number;
use num::{BigInt, CheckedAdd, One};

/// The values of Narayana's cows sequence. From a word problem in which a mature cow produces one cow per year and a cow takes four years to mature.
///
/// ```text
/// 1, 1, 1, 2, 3, 4, 6, 9, 13, 19, 28, 41, 60, 88, 129, 189, 277, 406...
/// ```
pub struct NarayanasCows<T> {
    a: T,
    b: T,
    c: T,
}

impl NarayanasCows<Number> {
    pub fn new() -> Self {
        Self { a: 1, b: 1, c: 1 }
    }
}

impl NarayanasCows<BigInt> {
    pub fn new_big() -> Self {
        Self {
            a: BigInt::one(),
            b: BigInt::one(),
            c: BigInt::one(),
        }
    }
}

impl<T: Clone + CheckedAdd> Iterator for NarayanasCows<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();

        let new = self.a.clone().checked_add(&self.c)?;
        self.a = self.b.clone();
        self.b = self.c.clone();
        self.c = new;

        Some(out)
    }
}

crate::check_sequences!(
    NarayanasCows::new(), [1, 1, 1, 2, 3, 4, 6, 9, 13, 19, 28, 41, 60, 88, 129, 189, 277, 406, 595, 872, 1278, 1873, 2745, 4023, 5896, 8641, 12664, 18560, 27201, 39865, 58425, 85626, 125491, 183916, 269542, 395033, 578949, 848491, 1243524, 1822473, 2670964, 3914488, 5736961, 8407925];
);

crate::sample_sequences!(
    NarayanasCows::new();
);
