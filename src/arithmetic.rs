use num::BigInt;

/// Arithmetic sequence with chosen initial value and increment
pub struct Arithmetic {
    val: BigInt,
    inc: BigInt,
}

impl Arithmetic {
    pub fn new(init: u64, n: u64) -> Self {
        Self {
            val: BigInt::from(init),
            inc: BigInt::from(n),
        }
    }
}

impl Iterator for Arithmetic {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val += &self.inc;
        Some(out)
    }
}

crate::print_a_few!(
    Arithmetic::new(0, 0), 0, 10;
    Arithmetic::new(0, 1), 0, 10;
    Arithmetic::new(0, 2), 0, 10;
    Arithmetic::new(0, 3), 0, 10;
);
