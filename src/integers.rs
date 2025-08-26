use num::{BigInt, One, Signed, Zero};

/// The integers
/// 0, 1, -1, 2, -2, 3, -3, 4, -4, 5...
pub struct Integer {
    val: BigInt,
    ctr: BigInt,
}

impl Integer {
    pub fn new() -> Self {
        Self {
            val: BigInt::zero(),
            ctr: BigInt::zero(),
        }
    }
}

impl Iterator for Integer {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.ctr = self.ctr.checked_add(&BigInt::one())?;
        if self.val.is_positive() {
            self.val = self.val.checked_sub(&self.ctr)?;
        } else {
            self.val = self.val.checked_add(&self.ctr)?;
        };
        Some(out)
    }
}

crate::print_a_few!(
    super::Integer::new(), 0, 10;
);
