use num::BigInt;

/// Arithmetic sequence with chosen initial value and increment
pub struct Arithmetic {
    val: BigInt,
    inc: BigInt,
}

impl Arithmetic {
    pub fn new<T>(init: T, inc: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            val: BigInt::from(init),
            inc: BigInt::from(inc),
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
    Arithmetic::new(1, 1), 0, 10, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    Arithmetic::new(3, 2), 0, 10, [3, 5, 7, 9, 11, 13, 15, 17, 19, 21];
    Arithmetic::new(4, 3), 0, 10, [4, 7, 10, 13, 16, 19, 22, 25, 28, 31];
);
