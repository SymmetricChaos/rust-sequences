use num::{BigInt, One, Zero};

/// The triangular numbers.
/// 0, 1, 3, 6, 10, 15, 21, 28, 36, 45...
pub struct Triangular {
    val: BigInt,
    ctr: BigInt,
}

impl Triangular {
    pub fn new_big() -> Self {
        Self {
            val: BigInt::zero(),
            ctr: BigInt::one(),
        }
    }

    /// The nth triangular number where T(0) = 0.
    /// Negative values are the generalized triangular numbers.
    pub fn nth<T>(n: T) -> BigInt
    where
        BigInt: From<T>,
    {
        let n = BigInt::from(n);
        (&n * (&n + 1)) / 2
    }
}

impl Iterator for Triangular {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val += &self.ctr;
        self.ctr += 1;
        Some(out)
    }

    // Optimized .nth() makes .skip() quicker
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.ctr += n;
        self.val = ((&self.ctr - 1) * &self.ctr) / 2;
        let out = self.val.clone();

        self.val += &self.ctr;
        self.ctr += 1;
        Some(out)
    }
}

crate::print_values!(
    Triangular::new_big(), 0, 10;
    Triangular::new_big(), 1, 10;
    Triangular::new_big(), 2, 10;
    Triangular::new_big(), 3, 10;
    Triangular::new_big(), 4, 10;
);

crate::check_sequences!(
    Triangular::new_big(), 0, 10, [0, 1, 3, 6, 10, 15, 21, 28, 36, 45];
);
