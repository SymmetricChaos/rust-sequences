use crate::{Number, core::traits::Increment};
use num::{BigInt, Integer, One, Zero};

/// The tetrahedral numbers. The partial sums of the triangular numbers.
///
/// ```text
/// 0, 1, 4, 10, 20, 35, 56, 84, 120, 165...
/// ```
pub struct Tetrahedral<T> {
    a: T,
    b: T,
    ctr: T,
}

impl Tetrahedral<Number> {
    pub fn new() -> Self {
        Self { a: 0, b: 1, ctr: 2 }
    }
}

#[cfg(feature = "big_int")]
impl Tetrahedral<BigInt> {
    pub fn new_big() -> Self {
        Self {
            a: BigInt::zero(),
            b: BigInt::one(),
            ctr: BigInt::from(2),
        }
    }

    pub fn nth<T>(n: T) -> BigInt
    where
        BigInt: From<T>,
    {
        let n = &BigInt::from(n);
        ((n + 1) * (n + 2) * (n + 3)) / 6
    }
}

impl Iterator for Tetrahedral<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a;
        let t = self
            .b
            .checked_add(self.b)?
            .checked_sub(self.a)?
            .checked_add(self.ctr)?;
        self.a = self.b;
        self.b = t;
        self.ctr.incr()?;
        Some(out)
    }
}

#[cfg(feature = "big_int")]
impl Iterator for Tetrahedral<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let t = &self.b + &self.b - &self.a + &self.ctr;
        self.a = self.b.clone();
        self.b = t;
        self.ctr.inc();
        Some(out)
    }
}

crate::check_sequences!(
    Tetrahedral::new_big(), [0, 1, 4, 10, 20, 35, 56, 84, 120, 165, 220, 286, 364, 455, 560, 680, 816, 969, 1140, 1330, 1540, 1771, 2024, 2300, 2600, 2925, 3276, 3654, 4060, 4495, 4960, 5456, 5984, 6545, 7140, 7770, 8436, 9139, 9880, 10660, 11480, 12341, 13244, 14190, 15180];
);
