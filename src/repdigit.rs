use num::{BigInt, CheckedAdd, CheckedMul, FromPrimitive, PrimInt, Zero};

/// The integers created by repeating the digits of an integer repeatedly
pub struct Repdigit<T> {
    digit: T,
    val: T,
    m: T,
}

impl<T: PrimInt> Repdigit<T> {
    /// Panics if digit is less than one.
    pub fn new(digit: T) -> Self {
        assert!(digit > T::zero());
        let ten = T::from(10).unwrap();
        let mut m = T::one();
        let mut d = digit;

        while !d.is_zero() {
            m = m * ten;
            d = d / ten;
        }

        Self {
            digit,
            val: T::zero(),
            m,
        }
    }
}

impl Repdigit<BigInt> {
    /// Panics if digit is less than one.
    pub fn new_big<N>(digit: N) -> Self
    where
        BigInt: From<N>,
    {
        let d = BigInt::from(digit);
        assert!(d > BigInt::zero());
        let m = BigInt::from_i32(10)
            .unwrap()
            .pow(d.to_string().len() as u32);
        Self {
            digit: d.clone(),
            val: BigInt::zero(),
            m,
        }
    }
}

impl<T: Clone + CheckedAdd + CheckedMul> Iterator for Repdigit<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.val = self.val.checked_mul(&self.m)?;
        self.val = self.val.checked_add(&self.digit)?;
        Some(self.val.clone())
    }
}

crate::check_sequences!(
    Repdigit::new_big(5), 0, 5, [5, 55, 555, 5555, 55555];
    Repdigit::new_big(12), 0, 5, [12, 1212, 121212, 12121212, 1212121212];
    Repdigit::new(7), 0, 5, [7, 77, 777, 7777, 77777];
    Repdigit::new(35), 0, 5, [35, 3535, 353535, 35353535]; // notice only four values returned because i32 overflowss
);
