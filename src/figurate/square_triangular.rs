use num::{BigInt, One, Zero};

/// The square triangular numbers.
/// 0, 1, 36, 1225, 41616, 1413721, 48024900...
pub struct SquareTriangular {
    a: BigInt,
    b: BigInt,
}

impl SquareTriangular {
    pub fn new_big() -> Self {
        Self {
            a: BigInt::zero(),
            b: BigInt::one(),
        }
    }
}

impl Iterator for SquareTriangular {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let t = (&self.b * 34 - &self.a) + 2;
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

crate::check_sequences!(
    SquareTriangular::new_big(), 0, 10, [0_i64, 1, 36, 1225, 41616, 1413721, 48024900, 1631432881, 55420693056, 1882672131025];
);
