use std::iter::Skip;

use num::{BigInt, CheckedAdd, CheckedMul, CheckedSub, One, PrimInt};

/// The factorial numbers.
/// 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800...
pub struct Factorial<T> {
    val: T,
    ctr: T,
}

impl<T: PrimInt> Factorial<T> {
    pub fn new_prim() -> Self {
        Self {
            val: T::one(),
            ctr: T::one() + T::one(),
        }
    }
}

impl Factorial<BigInt> {
    pub fn new() -> Self {
        Self {
            val: BigInt::from(1),
            ctr: BigInt::from(1),
        }
    }
}

impl<T: CheckedAdd + CheckedMul + Clone + One> Iterator for Factorial<T> {
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
pub struct AlternatingFactorial<T> {
    val: T,
    factorials: Skip<Factorial<T>>,
}

impl<T: PrimInt> AlternatingFactorial<T> {
    pub fn new_prim() -> Self {
        Self {
            val: T::one(),
            factorials: Factorial::<T>::new_prim().skip(2),
        }
    }
}

impl AlternatingFactorial<BigInt> {
    pub fn new() -> Self {
        Self {
            val: BigInt::from(1),
            factorials: Factorial::new().skip(2),
        }
    }
}

impl<T: CheckedMul + CheckedAdd + CheckedSub + Clone + One> Iterator for AlternatingFactorial<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.factorials.next()?.checked_sub(&self.val)?;
        Some(out)
    }
}

/// The double factorial numbers. The sum of the products of the odd naturals.
/// 1, 3, 15, 105, 945, 10395, 135135, 2027025, 34459425...
pub struct DoubleFactorial<T> {
    val: T,
    ctr: T,
}

impl<T: PrimInt> DoubleFactorial<T> {
    pub fn new_prim() -> Self {
        Self {
            val: T::one(),
            ctr: T::one() + T::one(),
        }
    }
}

impl DoubleFactorial<BigInt> {
    pub fn new() -> Self {
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
    Factorial::new(), 37_000;
);

crate::check_sequences!(
    Factorial::new(), 0, 10, [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
    DoubleFactorial::new(), 0, 10, [1, 1, 3, 15, 105, 945, 10395, 135135, 2027025, 34459425];
    AlternatingFactorial::new(), 0, 10, [1, 1, 5, 19, 101, 619, 4421, 35899, 326981, 3301819];
);
