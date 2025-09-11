use num::{BigInt, One};

/// The powers of n.
pub struct Powers {
    val: BigInt,
    n: BigInt,
}

impl Powers {
    pub fn new<T: Clone>(n: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            val: BigInt::one(),
            n: BigInt::from(n),
        }
    }
}

impl Iterator for Powers {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val *= &self.n;
        Some(out)
    }

    // Should be slightly faster than iteration with .next()
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.val *= self.n.pow(n.try_into().unwrap());
        let out = self.val.clone();
        self.val *= &self.n;
        Some(out)
    }
}

crate::print_values!(
    Powers::new(3), 5, 10;
);

crate::check_sequences!(
    Powers::new(-2), 0, 10, [1, -2, 4, -8, 16, -32, 64, -128, 256, -512];
    Powers::new(1), 0, 10, [1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    Powers::new(2), 0, 10, [1, 2, 4, 8, 16, 32, 64, 128, 256, 512];
    Powers::new(3), 0, 10, [1, 3, 9, 27, 81, 243, 729, 2187, 6561, 19683];
    Powers::new(4), 0, 10, [1, 4, 16, 64, 256, 1024, 4096, 16384, 65536, 262144];
);
