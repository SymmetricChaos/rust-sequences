use num::{BigInt, One};

/// The powers of n
pub struct Power {
    val: BigInt,
    n: BigInt,
}

impl Power {
    /// Panics if n = 0
    pub fn new(n: u64) -> Self {
        assert!(n != 0);
        Self {
            val: BigInt::one(),
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

crate::print_a_few!(
    Power::new(1), 0, 10;
    Power::new(2), 0, 10;
    Power::new(3), 0, 10;
    Power::new(4), 0, 10;
    Power::new(u64::MAX), 0, 4;
);
