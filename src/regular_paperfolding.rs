use num::{BigInt, CheckedAdd, CheckedDiv, Integer};

use crate::odd_part::OddPart;

/// The regular paperfolding sequence. Each term is 1 if the odd part of n is equal to 1 modulo 4 and otherwise is 0.
///
/// 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0...
pub struct RegularPaperfolding<T> {
    odd_part: OddPart<T>,
    four: T,
}

impl<T: Clone + Integer> RegularPaperfolding<T> {
    pub fn new() -> Self {
        Self {
            odd_part: OddPart::new(),
            four: T::one() + T::one() + T::one() + T::one(),
        }
    }
}

impl RegularPaperfolding<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Clone + Integer + CheckedDiv + CheckedAdd> Iterator for RegularPaperfolding<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.odd_part.next()?;

        if (n % self.four.clone()).is_one() {
            Some(T::one())
        } else {
            Some(T::zero())
        }
    }
}

crate::check_sequences!(
    RegularPaperfolding::<u32>::new(), [1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0];
);
