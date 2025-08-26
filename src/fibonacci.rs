use num::{BigInt, One, Zero};

/// The Fibonacci numbers
/// 0, 1, 1, 2, 3, 5, 8, 13, 21, 34...
pub struct Fibonacci {
    a: BigInt,
    b: BigInt,
}

impl Fibonacci {
    pub fn new() -> Self {
        Self {
            a: BigInt::zero(),
            b: BigInt::one(),
        }
    }
}

impl Iterator for Fibonacci {
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
    Fibonacci::new(), 0, 10;
);
