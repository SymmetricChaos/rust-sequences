use num::{BigInt, Signed, Zero};

/// The square numbers.
/// 0, 1, 4, 9, 16, 25, 36, 49, 64, 81...
pub struct Square {
    val: BigInt,
}

impl Square {
    pub fn new() -> Self {
        Self {
            val: BigInt::zero(),
        }
    }

    /// The nth square number
    /// Panics if n is negative.
    pub fn nth<T>(n: T) -> BigInt
    where
        BigInt: From<T>,
    {
        let n = BigInt::from(n);
        assert!(!n.is_negative());
        &n * &n
    }
}

impl Iterator for Square {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = &self.val * &self.val;
        self.val += 1;

        Some(out)
    }
}

crate::check_iteration_times!(
    Square::new(), 4_500_000;
);

crate::check_sequences!(
    Square::new(), 0, 10, [0, 1, 4, 9, 16, 25, 36, 49, 64, 81];
);
