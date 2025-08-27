use num::{BigInt, Signed};

/// The integers.
/// 0, 1, -1, 2, -2, 3, -3, 4, -4, 5...
pub struct Integer {
    val: BigInt,
    ctr: BigInt,
}

impl Integer {
    pub fn new() -> Self {
        Self {
            val: BigInt::from(0),
            ctr: BigInt::from(0),
        }
    }
}

impl Iterator for Integer {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.ctr += 1;
        if self.val.is_positive() {
            self.val -= &self.ctr;
        } else {
            self.val += &self.ctr;
        };
        Some(out)
    }
}

crate::print_a_few!(
    Integer::new(), 0, 10;
);
