use num::{BigInt, Zero};

/// The oblong numbers
/// 0, 2, 6, 12, 20, 30, 42, 56, 72, 90...
pub struct Oblong {
    val: BigInt,
    ctr: BigInt,
}

impl Oblong {
    pub fn new() -> Self {
        Self {
            val: BigInt::zero(),
            ctr: BigInt::from(2),
        }
    }
}

impl Iterator for Oblong {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.val.checked_add(&self.ctr)?;
        self.ctr = self.ctr.checked_add(&BigInt::from(2))?;
        Some(out)
    }
}

crate::print_a_few!(
    super::Oblong::new(), 0, 10;
);
