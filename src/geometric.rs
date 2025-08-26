use num::BigInt;

/// Geometric sequence with chosen initial value and multiplier
pub struct Geometric {
    val: BigInt,
    inc: BigInt,
}

impl Geometric {
    pub fn new(init: u64, n: u64) -> Self {
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

crate::print_a_few!(
    Geometric::new(3, 2), 0, 10;
    Geometric::new(4, 3), 0, 10;
);
