use num::BigInt;

/// Geometric sequence with chosen initial value and multiplier
pub struct Geometric {
    val: BigInt,
    inc: BigInt,
}

impl Geometric {
    pub fn new<T>(init: T, n: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            val: BigInt::from(init),
            inc: BigInt::from(n),
        }
    }
}

impl Iterator for Geometric {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val *= &self.inc;
        Some(out)
    }
}

crate::check_sequences!(
    Geometric::new(3, 2), 0, 10, [3, 6, 12, 24, 48, 96, 192, 384, 768, 1536];
    Geometric::new(4, 3), 0, 10, [4, 12, 36, 108, 324, 972, 2916, 8748, 26244, 78732];
);
