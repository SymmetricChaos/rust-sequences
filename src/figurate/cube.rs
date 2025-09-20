use num::{BigInt, Zero};

/// The cube numbers.
/// 0, 1, 8, 27, 64, 125, 216, 343, 512, 729...
pub struct Cube {
    val: BigInt,
}

impl Cube {
    pub fn new_big() -> Self {
        Self {
            val: BigInt::zero(),
        }
    }

    /// The nth cube number.
    pub fn nth<T>(n: T) -> BigInt
    where
        BigInt: From<T>,
    {
        let n = BigInt::from(n);
        &n * &n * &n
    }
}

impl Iterator for Cube {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = &self.val * &self.val * &self.val;
        self.val += 1;

        Some(out)
    }
}

crate::check_iteration_times!(
    Cube::new_big(), 4_500_000;
);

crate::check_sequences!(
    Cube::new_big(), 0, 10, [0, 1, 8, 27, 64, 125, 216, 343, 512, 729];
);
