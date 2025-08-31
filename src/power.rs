use num::BigInt;

/// The powers of n
pub struct Power {
    val: BigInt,
    n: BigInt,
}

impl Power {
    /// Panics if n <= 0
    pub fn new(n: i64) -> Self {
        assert!(n > 0);
        Self {
            val: BigInt::from(1),
            n: BigInt::from(n),
        }
    }
}

impl Iterator for Power {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val *= &self.n;
        Some(out)
    }
}

crate::check_sequences!(
    Power::new(1), 0, 10, [1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    Power::new(2), 0, 10, [1, 2, 4, 8, 16, 32, 64, 128, 256, 512];
    Power::new(3), 0, 10, [1, 3, 9, 27, 81, 243, 729, 2187, 6561, 19683];
    Power::new(4), 0, 10, [1, 4, 16, 64, 256, 1024, 4096, 16384, 65536, 262144];
);
