use crate::{Number, core::traits::Increment};
use num::{BigInt, Integer, Zero};

/// The square numbers.
///
/// ```text
/// f(n) = n*n
/// 0, 1, 4, 9, 16, 25, 36, 49, 64, 81...
/// ```
pub struct Square<T> {
    val: T,
}
impl Square<Number> {
    pub fn new() -> Self {
        Self { val: 0 }
    }
}

#[cfg(feature = "big_int")]
impl Square<BigInt> {
    pub fn new_big() -> Self {
        Self {
            val: BigInt::zero(),
        }
    }

    pub fn nth<T>(n: T) -> BigInt
    where
        BigInt: From<T>,
    {
        let n = &BigInt::from(n);
        n * n
    }
}

impl Iterator for Square<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.checked_mul(self.val)?;
        self.val.incr()?;
        Some(out)
    }
}

#[cfg(feature = "big_int")]
impl Iterator for Square<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = &self.val * &self.val;
        self.val.inc();
        Some(out)
    }
}

crate::check_iteration_times!(
    Square::new_big(), 4_500_000;
);

crate::check_sequences!(
    Square::new_big(), [0, 1, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144, 169, 196, 225, 256, 289, 324, 361, 400, 441, 484, 529, 576, 625, 676, 729, 784, 841, 900, 961, 1024, 1089, 1156, 1225, 1296, 1369, 1444, 1521, 1600, 1681, 1764, 1849, 1936, 2025, 2116, 2209, 2304, 2401, 2500];
);
