use crate::core::natural::Naturals;
use num::{BigInt, CheckedAdd, CheckedMul, One, Zero};

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
        Self::new(p)
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
    NthPowers::new_big(2), [0, 1, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144, 169, 196, 225, 256, 289, 324, 361, 400, 441, 484, 529, 576, 625, 676, 729, 784, 841, 900, 961, 1024, 1089, 1156, 1225, 1296, 1369, 1444, 1521, 1600, 1681, 1764, 1849, 1936, 2025, 2116, 2209, 2304, 2401, 2500];
);
