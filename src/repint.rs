use num::{BigInt, CheckedAdd, CheckedDiv, CheckedMul, Integer};

/// The integers created by repeating the digits of a positive integer.
pub struct Repint<T> {
    number: T,
    val: T,
    m: T,
}

impl<T: CheckedAdd + CheckedDiv + CheckedMul + Clone + Integer> Repint<T> {
    /// Panics if number is less than one or if base is greater than T::max()
    pub fn new(number: T, base: usize) -> Self {
        assert!(number > T::zero());

        // Just a way to convert the base from usize
        let mut b = T::zero();
        for _ in 0..base {
            b = b.checked_add(&T::one()).unwrap();
        }

        let mut m = T::one();
        let mut d = number.clone();

        while !d.is_zero() {
            m = m.checked_mul(&b).unwrap();
            d = d.checked_div(&b).unwrap();
        }

        Self {
            number,
            val: T::zero(),
            m,
        }
    }
}

impl Repint<BigInt> {
    /// Panics if number is less than one.
    pub fn new_big<N>(number: N, base: usize) -> Self
    where
        BigInt: From<N>,
    {
        Self::new(BigInt::from(number), base)
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
    Repint::new_big(5,10), [5, 55, 555, 5555, 55555];
    Repint::new_big(12,10), [12, 1212, 121212, 12121212, 1212121212];
    Repint::new(7,10), [7, 77, 777, 7777, 77777];
    Repint::new(35,10), [35, 3535, 353535, 35353535]; // notice only four values returned because i32 overflowss
);
