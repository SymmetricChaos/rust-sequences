use num::{BigInt, One};

use crate::Number;

/// The Leonardo numbers. Defined by a linear recurrence similar to the Fibonnaci numbers.
///
/// ```text
/// 1, 1, 3, 5, 9, 15, 25, 41, 67, 109, 177, 287, 465, 753, 1219, 1973...
/// ```
pub struct Leonardo<T> {
    a: T,
    b: T,
}

impl Leonardo<Number> {
    pub fn new() -> Self {
        Self { a: 1, b: 1 }
    }
}

impl Leonardo<BigInt> {
    pub fn new_big() -> Self {
        Self {
            a: BigInt::one(),
            b: BigInt::one(),
        }
    }
}

impl Iterator for Leonardo<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a;
        let t = self.a.checked_add(self.b)?.checked_add(1)?;
        self.a = self.b;
        self.b = t;
        Some(out)
    }
}

impl Iterator for Leonardo<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let t = &self.a + &self.b + 1;
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

crate::check_iteration_times!(
    Leonardo::new_big(), 160_000;
);

crate::check_sequences!(
    Leonardo::new_big(), [1, 1, 3, 5, 9, 15, 25, 41, 67, 109, 177, 287, 465, 753, 1219, 1973, 3193, 5167, 8361, 13529, 21891, 35421, 57313, 92735, 150049, 242785, 392835, 635621, 1028457, 1664079, 2692537, 4356617, 7049155, 11405773, 18454929, 29860703, 48315633, 78176337];
);

crate::sample_sequences!(
    Leonardo::new();
);
