use crate::{Number, utils::divisibility::prime_factorization};

/// The positive integers which are the product of exactly two prime numbers.
///
/// ```text
/// 4, 6, 9, 10, 14, 15, 21, 22, 25, 26, 33, 34, 35...
/// ```
pub struct Semiprime {
    ctr: Number,
}

impl Semiprime {
    pub fn new() -> Self {
        Self { ctr: 3 }
    }
}

impl Iterator for Semiprime {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.ctr += 1;
            let s: Number = prime_factorization(self.ctr).into_iter().map(|x| x.1).sum();
            if s == 2 {
                return Some(self.ctr);
            }
        }
    }
}

/// The positive integers which are the product of exactly k prime numbers (potentially with repetition).
///
/// ```text
/// k = 2 (semiprimes)
/// 4, 6, 9, 10, 14, 15, 21, 22, 25...
///
/// k = 3
/// 8, 12, 18, 20, 27, 28, 30, 42, 44...
/// ```
pub struct AlmostPrime {
    ctr: Number,
    k: Number,
}

impl AlmostPrime {
    /// k must be positive.
    pub fn new(k: Number) -> Self {
        assert!(k > 0);
        Self { ctr: 2, k }
    }
}

impl Iterator for AlmostPrime {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.ctr += 1;
            let s: Number = prime_factorization(self.ctr).into_iter().map(|x| x.1).sum();
            if s == self.k {
                return Some(self.ctr);
            }
        }
    }
}

crate::check_sequences!(
    Semiprime::new(),    [4, 6, 9, 10, 14, 15, 21, 22, 25, 26, 33, 34, 35, 38, 39, 46, 49, 51, 55, 57, 58, 62, 65, 69, 74, 77, 82, 85, 86, 87, 91, 93, 94, 95, 106, 111, 115, 118, 119, 121, 122, 123, 129, 133, 134, 141, 142, 143, 145, 146, 155, 158, 159, 161, 166, 169, 177, 178, 183, 185, 187];
    AlmostPrime::new(2), [4, 6, 9, 10, 14, 15, 21, 22, 25, 26, 33, 34, 35, 38, 39, 46, 49, 51, 55, 57, 58, 62, 65, 69, 74, 77, 82, 85, 86, 87, 91, 93, 94, 95, 106, 111, 115, 118, 119, 121, 122, 123, 129, 133, 134, 141, 142, 143, 145, 146, 155, 158, 159, 161, 166, 169, 177, 178, 183, 185, 187];
    AlmostPrime::new(3), [8, 12, 18, 20, 27, 28, 30, 42, 44, 45, 50, 52, 63, 66, 68, 70, 75, 76, 78, 92, 98, 99, 102, 105, 110, 114, 116, 117, 124, 125, 130, 138, 147, 148, 153, 154, 164, 165, 170, 171, 172, 174, 175, 182, 186, 188, 190, 195, 207, 212, 222, 230, 231, 236, 238, 242, 244];
);
