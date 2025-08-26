use num::{BigInt, One, Zero};

/// The triangular numbers.
/// 0, 1, 3, 6, 10, 15, 21, 28, 36, 45...
pub struct Triangular {
    val: BigInt,
    ctr: BigInt,
}

impl Triangular {
    pub fn new() -> Self {
        Self {
            val: BigInt::zero(),
            ctr: BigInt::one(),
        }
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

crate::print_a_few!(
    Triangular::new(), 0, 10;
);
