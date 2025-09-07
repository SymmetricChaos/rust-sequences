use num::{BigInt, Signed, Zero};

/// The natural numbers. The non-negative integers.
/// 0, 1, 2, 3, 4, 5, 6, 7, 8, 9...
pub struct Natural {
    ctr: BigInt,
}

impl Natural {
    pub fn new() -> Self {
        Self {
            ctr: BigInt::zero(),
        }
    }

    /// Natural numbers starting at a given value.
    /// Panics if n is negative.
    pub fn from<T>(n: T) -> Self
    where
        BigInt: From<T>,
    {
        let ctr = BigInt::from(n);
        assert!(!ctr.is_negative());
        Self { ctr }
    }
}

impl Iterator for Natural {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.ctr.clone();
        self.ctr += 1;
        Some(out)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.ctr += n;
        Some(self.ctr.clone())
    }
}

crate::check_sequences!(
    Natural::new(), 0, 10, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
);
