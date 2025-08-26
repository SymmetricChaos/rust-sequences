use num::{BigInt, One, Zero};

/// The Pell numbers
/// 0, 1, 2, 5, 12, 29, 70, 169, 408, 985...
pub struct Pell {
    a: BigInt,
    b: BigInt,
}

impl Pell {
    pub fn new() -> Self {
        Self {
            a: BigInt::zero(),
            b: BigInt::one(),
        }
    }
}

impl Iterator for Pell {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let t = &self.a + (&self.b * 2);
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

crate::print_a_few!(
    Pell::new(), 0, 10;
);
