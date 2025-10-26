use num::{BigInt, CheckedAdd, CheckedDiv, CheckedMul, FromPrimitive, One, Zero};

/// The integers created by repeating the digits of a positive integer.
/// The repints of 12
/// 12, 1212, 121212, 12121212...
pub struct Repint<T> {
    digit: T,
    val: T,
    m: T,
}

impl<T: CheckedAdd + CheckedDiv + CheckedMul + Clone + One + Ord + Zero> Repint<T> {
    /// Panics if digit is less than one or if base is greater than T::max()
    pub fn new(digit: T, base: usize) -> Self {
        assert!(digit > T::zero());
        let mut b = T::zero();
        for _ in 0..base {
            b = b.checked_add(&T::one()).unwrap();
        }
        let mut m = T::one();
        let mut d = digit.clone();

        while !d.is_zero() {
            m = m.checked_mul(&b).unwrap();
            d = d.checked_div(&b).unwrap();
        }

        Self {
            digit,
            val: T::zero(),
            m,
        }
    }
}

impl Repint<BigInt> {
    /// Panics if digit is less than one.
    pub fn new_big<N>(digit: N, base: usize) -> Self
    where
        BigInt: From<N>,
    {
        let d = BigInt::from(digit);
        assert!(d > BigInt::zero());
        let m = BigInt::from_usize(base)
            .unwrap()
            .pow(d.to_string().len() as u32);
        Self {
            digit: d.clone(),
            val: BigInt::zero(),
            m,
        }
    }
}

impl<T: CheckedAdd + CheckedMul + Clone> Iterator for Repint<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.val = self.val.checked_mul(&self.m)?;
        self.val = self.val.checked_add(&self.digit)?;
        Some(self.val.clone())
    }
}

crate::check_sequences!(
    Repint::new_big(5,10), 0, 5, [5, 55, 555, 5555, 55555];
    Repint::new_big(12,10), 0, 5, [12, 1212, 121212, 12121212, 1212121212];
    Repint::new(7,10), 0, 5, [7, 77, 777, 7777, 77777];
    Repint::new(35,10), 0, 5, [35, 3535, 353535, 35353535]; // notice only four values returned because i32 overflowss
);
