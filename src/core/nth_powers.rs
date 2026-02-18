use num::{BigInt, CheckedAdd, CheckedMul, One, Zero};

use crate::core::natural::Naturals;

/// The natural numbers raised to a fixed power.
pub struct NthPowers<T> {
    nats: Naturals<T>,
    p: usize,
}

impl<T: CheckedAdd + CheckedMul + Clone + One + Zero> NthPowers<T> {
    pub fn new(p: u32) -> Self {
        Self {
            nats: Naturals::<T>::new(),
            p: usize::try_from(p).expect("failed to convert u32 to usize"),
        }
    }
}

impl NthPowers<BigInt> {
    pub fn new_big(p: u32) -> Self {
        Self {
            nats: Naturals::new_big(),
            p: usize::try_from(p).expect("failed to convert u32 to usize"),
        }
    }
}

impl<T: CheckedAdd + CheckedMul + Clone + One> Iterator for NthPowers<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        num::checked_pow(self.nats.next()?, self.p)
    }
}

crate::check_sequences!(
    NthPowers::new_big(1), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    NthPowers::new_big(2), [0, 1, 4, 9, 16, 25, 36, 49, 64, 81];
);

crate::print_values!(
    NthPowers::<u32>::new(12), 0, 10;
);
