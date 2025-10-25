use std::iter::Skip;

use num::{BigInt, CheckedAdd, CheckedMul, CheckedSub, One};

/// The factorial numbers.
/// 1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800...
pub struct Factorials<T> {
    val: T,
    ctr: T,
}

impl<T: CheckedAdd + CheckedMul + Clone + One> Factorials<T> {
    pub fn new() -> Self {
        Self {
            val: T::one(),
            ctr: T::one(),
        }
    }
}

impl Factorials<BigInt> {
    pub fn new_big() -> Self {
        Self {
            val: BigInt::from(1),
            ctr: BigInt::from(1),
        }
    }
}

impl<T: CheckedAdd + CheckedMul + Clone + One> Iterator for Factorials<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.val.checked_mul(&self.ctr)?;
        self.ctr = self.ctr.checked_add(&T::one())?;
        Some(out)
    }
}

/// The alternating factorial numbers.
/// 1, 1, 5, 19, 101, 619, 4421, 35899, 326981...
pub struct AlternatingFactorials<T> {
    val: T,
    factorials: Skip<Factorials<T>>,
}

impl<T: CheckedAdd + CheckedMul + CheckedSub + Clone + One> AlternatingFactorials<T> {
    pub fn new() -> Self {
        Self {
            val: T::one(),
            factorials: Factorials::<T>::new().skip(2),
        }
    }
}

impl AlternatingFactorials<BigInt> {
    pub fn new_big() -> Self {
        Self {
            val: BigInt::from(1),
            factorials: Factorials::new_big().skip(2),
        }
    }
}

impl<T: CheckedMul + CheckedAdd + CheckedSub + Clone + One> Iterator for AlternatingFactorials<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.factorials.next()?.checked_sub(&self.val)?;
        Some(out)
    }
}

/// The double factorial numbers. The sum of the products of the odd naturals.
/// 1, 1, 3, 15, 105, 945, 10395, 135135, 2027025, 34459425...
pub struct DoubleFactorial<T> {
    val: T,
    ctr: T,
}

impl<T: CheckedAdd + CheckedMul + Clone + One> DoubleFactorial<T> {
    pub fn new() -> Self {
        Self {
            val: T::one(),
            ctr: T::one(),
        }
    }
}

impl DoubleFactorial<BigInt> {
    pub fn new_big() -> Self {
        Self {
            val: BigInt::from(1),
            ctr: BigInt::from(1),
        }
    }
}

impl<T: CheckedAdd + CheckedMul + Clone + One> Iterator for DoubleFactorial<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.val.checked_mul(&self.ctr)?;
        self.ctr = self.ctr.checked_add(&T::one())?;
        self.ctr = self.ctr.checked_add(&T::one())?;
        Some(out)
    }
}

crate::check_iteration_times!(
    Factorials::new_big(), 1_000;
);

crate::check_sequences!(
    Factorials::new_big(), 0, 10, [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
    Factorials::<i32>::new(), 0, 10, [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
    DoubleFactorial::new_big(), 0, 10, [1, 1, 3, 15, 105, 945, 10395, 135135, 2027025, 34459425];
    DoubleFactorial::<i32>::new(), 0, 10, [1, 1, 3, 15, 105, 945, 10395, 135135, 2027025, 34459425];
    AlternatingFactorials::new_big(), 0, 10, [1, 1, 5, 19, 101, 619, 4421, 35899, 326981, 3301819];
    AlternatingFactorials::<i32>::new(), 0, 10, [1, 1, 5, 19, 101, 619, 4421, 35899, 326981, 3301819];
);
