use crate::{core::composite::Composites, utils::divisibility::sum_of_divisors};
use num::integer::gcd;

/// Duffian numbers. Composite numbers that are coprime to the sum of their divisors.
///
/// 4, 8, 9, 16, 21, 25, 27, 32, 35, 36...
pub struct Duffian {
    c: Composites<u64>,
}

impl Duffian {
    pub fn new() -> Self {
        Self {
            c: Composites::new(),
        }
    }
}

impl Iterator for Duffian {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let n = self.c.next()?;
            let sigma = sum_of_divisors(n)?;
            if gcd(n, sigma) == 1 {
                return Some(n);
            }
        }
    }
}

crate::check_sequences!(
    Duffian::new(), [4, 8, 9, 16, 21, 25, 27, 32, 35, 36, 39, 49, 50, 55, 57, 63, 64, 65, 75, 77, 81, 85, 93, 98, 100, 111, 115, 119, 121, 125, 128, 129, 133, 143, 144, 155, 161, 169, 171, 175, 183, 185, 187, 189, 201, 203, 205, 209, 215, 217, 219, 221, 225, 235, 237, 242, 243, 245, 247];
);
