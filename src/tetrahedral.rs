use num::BigInt;

/// The tetrahedral numbers.
/// 0, 1, 4, 10, 20, 35, 56, 84, 120, 165...
pub struct Tetrahedral {
    a: BigInt,
    b: BigInt,
    ctr: BigInt,
}

impl Tetrahedral {
    pub fn new() -> Self {
        Self {
            a: BigInt::from(0),
            b: BigInt::from(1),
            ctr: BigInt::from(2),
        }
    }
}

impl Iterator for Tetrahedral {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let t = 2 * &self.b - &self.a + &self.ctr;
        self.a = self.b.clone();
        self.b = t;
        self.ctr += 1;
        Some(out)
    }
}

crate::print_a_few!(
    Tetrahedral::new(), 0, 10;
);
