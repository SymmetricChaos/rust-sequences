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

crate::check_sequences!(
    Arithmetic::new(0, 0), 0, 10, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    Arithmetic::new(0, 1), 0, 10, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    Arithmetic::new(0, 2), 0, 10, [0, 2, 4, 6, 8, 10, 12, 14, 16, 18];
    Arithmetic::new(0, 3), 0, 10, [0, 3, 6, 9, 12, 15, 18, 21, 24, 27];
);
