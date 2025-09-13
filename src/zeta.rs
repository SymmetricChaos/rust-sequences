use crate::core::{IntegerReciprocals, NthPowers, PartialSums};
use num::BigRational;

/// The partial sums of the Riemann zeta function for natural numbers greater than zero
pub struct Zeta {
    ctr: PartialSums<BigRational>,
}

impl Zeta {
    /// Power p is specified as a u32 due to the interface of the .pow() function.
    pub fn new(p: u32) -> Self {
        Self {
            ctr: PartialSums::new(IntegerReciprocals::new(NthPowers::new(p).skip(1))),
        }
    }
}

impl Iterator for Zeta {
    type Item = BigRational;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.next()
    }
}

crate::print_values!(
    Zeta::new(2), 0, 8; // converges to e
);
