use num::{BigInt, CheckedAdd, CheckedMul, CheckedSub, One};

/// Sylvester's sequence. Starting at 2 each term is the product of all previous terms plus one.
/// 2, 3, 7, 43, 1807, 3263443...
pub struct Sylvester<T> {
    current: T,
}

impl<T: Clone + One + CheckedMul + CheckedAdd> Sylvester<T> {
    pub fn new() -> Self {
        Self {
            current: T::one() + T::one(),
        }
    }
}

impl Sylvester<BigInt> {
    pub fn new_big() -> Self {
        Self {
            current: BigInt::one() + BigInt::one(),
        }
    }
}

impl<T: Clone + One + CheckedMul + CheckedAdd + CheckedSub> Iterator for Sylvester<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.current.clone();

        self.current = self
            .current
            .checked_mul(&(self.current.checked_sub(&T::one()))?)?
            .checked_add(&T::one())?;

        Some(out)
    }
}

crate::check_sequences!(
    Sylvester::new_big(), [2, 3, 7, 43, 1807, 3263443];
);
