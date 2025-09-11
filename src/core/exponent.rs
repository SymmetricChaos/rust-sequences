use num::BigInt;

use crate::core::Natural;

/// The natural numbers raised to a fixed power.
pub struct Exponent {
    nats: Natural,
    p: u32,
}

impl Exponent {
    /// Power p is specified as a u32 due to the interface of the .pow() function.
    pub fn new(p: u32) -> Self {
        Self {
            nats: Natural::new(),
            p,
        }
    }
}

impl Iterator for Exponent {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.nats.next().unwrap().pow(self.p))
    }
}

crate::check_sequences!(
    Exponent::new(1), 0, 10, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    Exponent::new(2), 0, 10, [0, 1, 4, 9, 16, 25, 36, 49, 64, 81];
);
