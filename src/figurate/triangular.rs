use crate::{Number, core::traits::Increment};
use num::{BigInt, CheckedAdd, FromPrimitive, Integer, One, Signed, Zero};

/// The triangular numbers. The partial sums of the natural numbers.
///
/// ```text
/// 0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 66, 78, 91, 105, 120, 136...
/// ```
pub struct Triangular<T> {
    val: T,
    ctr: T,
}

impl Triangular<Number> {
    pub fn new() -> Self {
        Self { val: 0, ctr: 1 }
    }
}

#[cfg(feature = "big_int")]
impl Triangular<BigInt> {
    pub fn new_big() -> Self {
        Self {
            val: BigInt::zero(),
            ctr: BigInt::one(),
        }
    }

    pub fn nth<T>(n: T) -> BigInt
    where
        BigInt: From<T>,
    {
        let n = BigInt::from(n);
        assert!(!n.is_negative());
        (n.clone() * (n + 1)) / BigInt::from_i32(2).unwrap()
    }
}

impl<T: Clone + CheckedAdd + Integer> Iterator for Triangular<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.val.checked_add(&self.ctr)?;
        self.ctr.incr()?;
        Some(out)
    }
}

crate::check_sequences!(
    Triangular::new_big(), [0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 66, 78, 91, 105, 120, 136, 153, 171, 190, 210, 231, 253, 276, 300, 325, 351, 378, 406, 435, 465, 496, 528, 561, 595, 630, 666, 703, 741, 780, 820, 861, 903, 946, 990, 1035, 1081, 1128, 1176, 1225, 1275, 1326, 1378, 1431];
);

crate::sample_sequences!(
    Triangular::new_big();
);
