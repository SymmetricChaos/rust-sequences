use num::{BigInt, CheckedAdd, CheckedMul, One, PrimInt};

use crate::core::Natural;

/// The natural numbers raised to a fixed power.
pub struct NthPowers<T> {
    nats: Natural<T>,
    p: usize,
}

impl<T: PrimInt> NthPowers<T> {
    pub fn new_prim(p: u32) -> Self {
        Self {
            nats: Natural::<T>::new_prim(),
            p: usize::try_from(p).expect("failed to convern u32 to usize"),
        }
    }
}

impl NthPowers<BigInt> {
    pub fn new(p: u32) -> Self {
        Self {
            nats: Natural::new(),
            p: usize::try_from(p).expect("failed to convern u32 to usize"),
        }
    }
}

impl<T: CheckedAdd + Clone + One + CheckedMul> Iterator for NthPowers<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        num::checked_pow(self.nats.next()?, self.p)
    }
}

crate::check_sequences!(
    NthPowers::new(1), 0, 10, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    NthPowers::new(2), 0, 10, [0, 1, 4, 9, 16, 25, 36, 49, 64, 81];
);

crate::print_values!(
    NthPowers::<u32>::new_prim(12), 0, 10;
);
