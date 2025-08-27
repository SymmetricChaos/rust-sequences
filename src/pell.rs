use num::BigInt;

/// The Pell numbers.
/// 0, 1, 2, 5, 12, 29, 70, 169, 408, 985...
pub struct Pell {
    a: BigInt,
    b: BigInt,
}

impl Pell {
    pub fn new() -> Self {
        Self {
            a: BigInt::from(0),
            b: BigInt::from(1),
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
