use num::{BigInt, CheckedAdd, CheckedMul, One, Zero};

/// The cube numbers.
/// 0, 1, 8, 27, 64, 125, 216, 343, 512, 729...
pub struct Cube<T> {
    val: T,
}

impl<T: CheckedMul + CheckedAdd + One + Zero> Cube<T> {
    pub fn new() -> Self {
        Self { val: T::zero() }
    }
}

impl Cube<BigInt> {
    pub fn new_big() -> Self {
        Self {
            val: BigInt::zero(),
        }
    }
}

impl<T: CheckedMul + CheckedAdd + One> Iterator for Cube<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.checked_mul(&self.val)?.checked_mul(&self.val)?;
        self.val = self.val.checked_add(&T::one())?;

        Some(out)
    }
}

crate::check_iteration_times!(
    Cube::new_big(), 4_500_000;
);

crate::check_sequences!(
    Cube::new_big(), [0, 1, 8, 27, 64, 125, 216, 343, 512, 729];
    Cube::<i32>::new(), [0, 1, 8, 27, 64, 125, 216, 343, 512, 729];
);
