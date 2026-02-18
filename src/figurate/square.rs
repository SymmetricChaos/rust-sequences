use num::{BigInt, CheckedAdd, CheckedMul, One, Zero};

/// The square numbers.
/// 0, 1, 4, 9, 16, 25, 36, 49, 64, 81...
pub struct Square<T> {
    val: T,
}
impl<T: Clone + CheckedAdd + CheckedMul + One + Zero> Square<T> {
    pub fn new() -> Self {
        Self { val: T::zero() }
    }
}

impl Square<BigInt> {
    pub fn new_big() -> Self {
        Self {
            val: BigInt::zero(),
        }
    }
}

impl<T: Clone + CheckedAdd + CheckedMul + One> Iterator for Square<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.checked_mul(&self.val)?;
        self.val = self.val.checked_add(&T::one())?;

        Some(out)
    }
}

crate::check_iteration_times!(
    Square::new_big(), 4_500_000;
);

crate::check_sequences!(
    Square::new_big(), [0, 1, 4, 9, 16, 25, 36, 49, 64, 81];
);
