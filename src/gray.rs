use num::PrimInt;

use crate::core::traits::Increment;

/// The Gray codes, bit sequence where each term differs from the previous by a single bit. A permutation of the integers.
///
/// 0, 1, 3, 2, 6, 7, 5, 4, 12, 13...
pub struct Gray<T> {
    ctr: T,
}

impl<T: PrimInt> Gray<T> {
    pub fn new() -> Self {
        Self { ctr: T::zero() }
    }
}

// impl Gray<BigInt> {
//     pub fn new_big() -> Self {
//         Self::new()
//     }
// }

impl<T: PrimInt> Iterator for Gray<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.ctr.clone() ^ (self.ctr >> 1);
        self.ctr.incr()?;
        Some(out)
    }
}

crate::check_sequences!(
    // Gray::new_big(), [0, 1, 3, 2, 6, 7, 5, 4, 12, 13];
    Gray::<i8>::new(), [0, 1, 3, 2, 6, 7, 5, 4, 12, 13];
);
