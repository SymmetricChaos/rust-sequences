use crate::sylvester::Sylvester;
use num::{BigInt, CheckedAdd, CheckedMul, CheckedSub, Integer, Zero, rational::Ratio};

/// Rational convergents of Cahen's constant. Partial sums of the reciprocals of the even terms of Sylvester's sequence.
///
/// The constant is a trancendental number equal to approximately 0.643410546288...
pub struct Cahen<T> {
    sylvester: Sylvester<T>,
    sum: Ratio<T>,
    overflowed: bool,
}

impl<T: Clone + Integer + CheckedAdd + CheckedMul + CheckedSub> Cahen<T> {
    pub fn new() -> Self {
        Self {
            sylvester: Sylvester::new(),
            sum: Ratio::zero(),
            overflowed: false,
        }
    }
}

impl Cahen<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Clone + Integer + CheckedAdd + CheckedMul + CheckedSub> Iterator for Cahen<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.overflowed {
            return None;
        }
        let out = self.sum.clone();
        match self.sylvester.next() {
            Some(n) => match self.sum.checked_add(&Ratio::new(T::one(), n)) {
                Some(s) => self.sum = s,
                None => {
                    self.overflowed = true;
                    return Some(out);
                }
            },
            None => {
                self.overflowed = true;
                return Some(out);
            }
        }
        self.sylvester.next();
        Some(out)
    }
}

#[cfg(test)]
use crate::core::traits::DigitSequence;
crate::print_sequences!(
    Cahen::new_big(), 5;
    Cahen::new_big().map(|q| q.digits(15).unwrap()), 5;
);
