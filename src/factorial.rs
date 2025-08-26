use num::{BigInt, One};

/// The factorial numbers
/// 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800...
pub struct Factorial {
    val: BigInt,
    ctr: BigInt,
}

impl Factorial {
    pub fn new() -> Self {
        Self {
            val: BigInt::one(),
            ctr: BigInt::from(2),
        }
    }
}

impl Iterator for Factorial {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.val.checked_mul(&self.ctr)?;
        self.ctr = self.ctr.checked_add(&BigInt::one())?;
        Some(out)
    }
}

crate::print_a_few!(
    super::Factorial::new(), 0, 10;
);
