use num::{BigInt, One, Zero};

/// The square triangular numbers.
///
/// ```text
/// 0, 1, 36, 1225, 41616, 1413721, 48024900...
/// ```
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
    SquareTriangular::new_big(), [0_u128, 1, 36, 1225, 41616, 1413721, 48024900, 1631432881, 55420693056, 1882672131025, 63955431761796, 2172602007770041, 73804512832419600, 2507180834294496361, 85170343853180456676, 2893284510173841030625, 98286503002057414584576, 3338847817559778254844961, 113422539294030403250144100];
);
