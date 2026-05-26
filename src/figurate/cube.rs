use num::{BigInt, CheckedAdd, CheckedMul, One, Zero};

/// The cube numbers.
///
/// ```text
/// f(n) = n*n*n
/// 0, 1, 8, 27, 64, 125, 216, 343, 512, 729...
/// ```
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
        Self::new()
    }

    pub fn nth<T>(n: T) -> BigInt
    where
        BigInt: From<T>,
    {
        let n = &BigInt::from(n);
        n * n * n
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
    Cube::new_big(), [0, 1, 8, 27, 64, 125, 216, 343, 512, 729, 1000, 1331, 1728, 2197, 2744, 3375, 4096, 4913, 5832, 6859, 8000, 9261, 10648, 12167, 13824, 15625, 17576, 19683, 21952, 24389, 27000, 29791, 32768, 35937, 39304, 42875, 46656, 50653, 54872, 59319, 64000, 68921, 74088, 79507];
);
