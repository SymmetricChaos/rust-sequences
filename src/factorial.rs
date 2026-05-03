use num::{BigInt, CheckedAdd, CheckedMul, CheckedSub, One};
use std::iter::Skip;

/// The factorial numbers. Each term is the previous term multiplied by the next positive integer. Equivalently f(n) = f(n-1) * n.
///
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

/// The alternating factorial numbers. The partial sums of the alternating sum of the factorials. Equivalently f(n) = n! - f(n-1).
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
        Self::new()
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
    Factorials::new_big(), [1_u128, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800, 39916800, 479001600, 6227020800, 87178291200, 1307674368000, 20922789888000, 355687428096000, 6402373705728000, 121645100408832000, 2432902008176640000, 51090942171709440000, 1124000727777607680000];
    Factorials::<i32>::new(), [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
    DoubleFactorial::new_big(), [1, 1, 3, 15, 105, 945, 10395, 135135, 2027025, 34459425];
    DoubleFactorial::<i32>::new(), [1, 1, 3, 15, 105, 945, 10395, 135135, 2027025, 34459425];
    AlternatingFactorials::new_big(), [1, 1, 5, 19, 101, 619, 4421, 35899, 326981, 3301819];
    AlternatingFactorials::<i32>::new(), [1, 1, 5, 19, 101, 619, 4421, 35899, 326981, 3301819];
);
