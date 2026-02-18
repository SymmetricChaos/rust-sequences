use num::{BigInt, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, One};

/// The Catalan numbers.
/// 1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862...
pub struct Catalan<T> {
    val: T,
    ctr: T,
}

impl<T: Clone + CheckedAdd + CheckedMul + CheckedSub + CheckedDiv + One> Catalan<T> {
    pub fn new() -> Self {
        Self {
            val: T::one(),
            ctr: T::one(),
        }
    }
}

impl Catalan<BigInt> {
    pub fn new_big() -> Self {
        Self {
            val: BigInt::one(),
            ctr: BigInt::one(),
        }
    }
}

impl<T: Clone + CheckedAdd + CheckedMul + CheckedSub + CheckedDiv + One> Iterator for Catalan<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        let two = T::one() + T::one();
        self.val = self
            .ctr
            .checked_mul(&two)?
            .checked_sub(&T::one())?
            .checked_mul(&two)?
            .checked_mul(&self.val)?
            .checked_div(&(self.ctr.checked_add(&T::one())?))?;
        self.ctr = self.ctr.checked_add(&T::one())?;
        Some(out)
    }
}

crate::check_iteration_times!(
    Catalan::new_big(), 60_000;
);

crate::check_sequences!(
    Catalan::new_big(), [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];
);
