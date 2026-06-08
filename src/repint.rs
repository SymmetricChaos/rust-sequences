use crate::Number;
use num::{BigInt, CheckedAdd, CheckedMul, FromPrimitive, One, Signed, Zero};

/// The integers created by repeating the digits of a positive integer.
///
/// ```text
/// number = 5, base = 10
/// 5, 55, 555, 5555, 55555, 555555, 5555555, 55555555, 555555555...
///
/// number = 23, base = 10
/// 23, 2323, 232323, 23232323, 2323232323, 232323232323...
///
/// number = 4, base = 8
/// 4, 36, 292, 2340, 18724, 149796, 1198372, 9586980, 76695844...
/// ```
pub struct Repint<T> {
    number: T,
    val: T,
    m: T,
}

impl Repint<Number> {
    pub fn new(number: Number, base: Number) -> Self {
        assert!(number > 0);
        assert!(base > 2);

        let mut m = 1;
        let mut d = number.clone();

        while !d.is_zero() {
            m *= base;
            d /= base;
        }

        Self { number, val: 0, m }
    }
}

#[cfg(feature = "big_int")]
impl Repint<BigInt> {
    pub fn new_big<N>(number: N, base: N) -> Self
    where
        BigInt: From<N>,
    {
        let number = BigInt::from(number);
        let base = BigInt::from(base);
        assert!(number.is_positive());
        assert!(base > BigInt::from_i32(2).unwrap());

        let mut m = BigInt::one();
        let mut d = number.clone();

        while !d.is_zero() {
            m *= &base;
            d /= &base;
        }

        Self {
            number,
            val: BigInt::zero(),
            m,
        }
    }
}

impl<T: CheckedAdd + CheckedMul + Clone> Iterator for Repint<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.val = self.val.checked_mul(&self.m)?;
        self.val = self.val.checked_add(&self.number)?;
        Some(self.val.clone())
    }
}

crate::check_sequences!(
    Repint::new_big(5, 10), [5, 55, 555, 5555, 55555];
    Repint::new_big(12, 10), [12, 1212, 121212, 12121212, 1212121212];
    Repint::new(7, 10), [7, 77, 777, 7777, 77777];
    Repint::new(35, 10), [35, 3535, 353535, 35353535]; // notice only four values returned because i32 overflowss
);

crate::sample_sequences!(
    Repint::new_big(5, 10);
    Repint::new_big(23, 10);
    Repint::new_big(4, 8);
);
