use crate::utils::divisibility::aliquot_sum;

/// The deficient numbers, those which have an aliquot sum less than themselves.
///
/// 1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 13, 14...
pub struct Deficient {
    n: u64,
}

impl Deficient {
    /// Only u64 output supported.
    pub fn new() -> Self {
        Self { n: 0 }
    }
}

impl Iterator for Deficient {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.n = self.n.checked_add(1)?;
        while aliquot_sum(self.n).unwrap() >= self.n {
            self.n += 1;
        }
        Some(self.n)
    }
}

crate::check_sequences!(
    Deficient::new(), [1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 13, 14, 15, 16, 17, 19, 21, 22, 23, 25, 26, 27, 29, 31, 32, 33, 34, 35, 37, 38, 39, 41, 43, 44, 45, 46, 47, 49, 50, 51, 52, 53, 55, 57, 58, 59, 61, 62, 63, 64, 65, 67, 68, 69, 71, 73, 74, 75, 76, 77, 79, 81, 82, 83, 85, 86];
);
