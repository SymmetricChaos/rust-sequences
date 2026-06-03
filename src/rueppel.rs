use crate::{Number, core::catalan::Catalan};
use num::{BigInt, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Integer};

/// The Rueppel sequence. The parity of the Catalan numbers.
///
/// ```text
/// 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0...
/// ```
pub struct Rueppel<T> {
    catalan: Catalan<T>,
}

impl Rueppel<Number> {
    pub fn new() -> Self {
        Self {
            catalan: Catalan::new(),
        }
    }
}

impl Rueppel<BigInt> {
    pub fn new_big() -> Self {
        Self {
            catalan: Catalan::new_big(),
        }
    }
}

impl<T: Clone + CheckedAdd + CheckedMul + CheckedSub + CheckedDiv + Integer> Iterator
    for Rueppel<T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.catalan.next().map(|n| n % (T::one() + T::one()))
    }
}

crate::check_sequences!(
    Rueppel::new_big(), [1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
);
