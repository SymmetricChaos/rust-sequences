use num::BigInt;

/// The triangular numbers.
/// 0, 1, 3, 6, 10, 15, 21, 28, 36, 45...
pub struct Triangular {
    val: BigInt,
    ctr: BigInt,
}

impl Triangular {
    pub fn new() -> Self {
        Self {
            val: BigInt::from(0),
            ctr: BigInt::from(1),
        }
    }

    /// The nth triangular number
    pub fn nth(n: u64) -> BigInt {
        (BigInt::from(n) * (BigInt::from(n) + 1)) / 2
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
}

crate::check_sequences!(
    Triangular::new(), 0, 10, [0, 1, 3, 6, 10, 15, 21, 28, 36, 45];
);
