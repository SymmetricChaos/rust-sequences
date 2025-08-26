use num::{BigInt, One};

/// The Lucas numbers.
/// 2, 1, 3, 4, 7, 11, 18, 29, 47, 76...
pub struct Lucas {
    a: BigInt,
    b: BigInt,
}

impl Lucas {
    pub fn new() -> Self {
        Self {
            a: BigInt::from(2),
            b: BigInt::one(),
        }
    }
}

impl Iterator for Lucas {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let t = &self.a + &self.b;
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

crate::print_a_few!(
    Lucas::new(), 0, 10;
);
