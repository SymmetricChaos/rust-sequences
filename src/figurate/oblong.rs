use num::BigInt;

/// The oblong numbers.
/// 0, 2, 6, 12, 20, 30, 42, 56, 72, 90...
pub struct Oblong {
    val: BigInt,
    ctr: BigInt,
}

impl Oblong {
    pub fn new() -> Self {
        Self {
            val: BigInt::from(0),
            ctr: BigInt::from(2),
        }
    }

    pub fn nth(n: u64) -> BigInt {
        BigInt::from(n) * (BigInt::from(n) + 1)
    }
}

impl Iterator for Oblong {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val += &self.ctr;
        self.ctr += 2;
        Some(out)
    }
}

crate::check_sequences!(
    Oblong::new(), 0, 10, [0, 2, 6, 12, 20, 30, 42, 56, 72, 90];
);
