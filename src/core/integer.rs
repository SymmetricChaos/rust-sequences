use num::{BigInt, Signed};

/// The integers in the canonical ordering.
/// 0, 1, -1, 2, -2, 3, -3, 4, -4, 5...
pub struct Integer {
    val: BigInt,
}

impl Integer {
    pub fn new() -> Self {
        Self {
            val: BigInt::from(0),
        }
    }
}

impl Iterator for Integer {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        if self.val.is_positive() {
            self.val *= BigInt::from(-1);
        } else {
            self.val *= BigInt::from(-1);
            self.val += 1;
        };
        Some(out)
    }
}

crate::check_times!(Integer::new(), 3_500_000);

crate::check_sequences!(
    Integer::new(), 0, 10, [0, 1, -1, 2, -2, 3, -3, 4, -4, 5];
);
