use num::BigInt;

/// The generalized pentagonal numbers.
pub struct GeneralizedPentagonal {
    integers: crate::core::Integer,
}

impl GeneralizedPentagonal {
    pub fn new() -> Self {
        Self {
            integers: crate::core::Integer::new(),
        }
    }
}

impl Iterator for GeneralizedPentagonal {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let k = self.integers.next()?;
        Some((&k * (3 * &k - 1)) / 2)
    }

    // Custom nth implementation skips intermediate calculations
    // This also makes .skip() faster
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        for _ in 0..n {
            self.integers.next();
        }
        let k = self.integers.next()?;
        Some((&k * (3 * &k - 1)) / 2)
    }
}

crate::check_sequences!(
    GeneralizedPentagonal::new(), 0, 10, [0, 1, 2, 5, 7, 12, 15, 22, 26, 35];
    GeneralizedPentagonal::new(), 10, 10, [40, 51, 57, 70, 77, 92, 100, 117, 126, 145];
);
