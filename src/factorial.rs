use crate::{Number, core::traits::Increment};
use num::{BigInt, CheckedAdd, CheckedMul, Integer, One};
use std::iter::Skip;

/// The factorial numbers. Each term is the previous term multiplied by the next positive integer. Equivalently f(n) = f(n-1) * n.
///
/// ```text
/// 1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800, 39916800...
/// ```
pub struct Factorial<T> {
    val: T,
    ctr: T,
}

impl Factorial<Number> {
    pub fn new() -> Self {
        Self { val: 1, ctr: 1 }
    }
}

#[cfg(feature = "big_int")]
impl Factorial<BigInt> {
    pub fn new_big() -> Self {
        Self {
            val: BigInt::one(),
            ctr: BigInt::one(),
        }
    }
}

impl<T: Clone + CheckedAdd + CheckedMul + Integer> Iterator for Factorial<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.val.checked_mul(&self.ctr)?;
        self.ctr.incr()?;
        Some(out)
    }
}

/// The alternating factorial numbers. The partial sums of the alternating sum of the factorials. Equivalently f(n) = n! - f(n-1).
///
/// ```text
/// 1, 1, 3, 15, 105, 945, 10395, 135135, 2027025, 34459425, 654729075...
/// ```
pub struct AlternatingFactorial<T> {
    val: T,
    factorials: Skip<Factorial<T>>,
}

impl AlternatingFactorial<Number> {
    pub fn new() -> Self {
        Self {
            val: 1,
            factorials: Factorial::new().skip(2),
        }
    }
}

#[cfg(feature = "big_int")]
impl AlternatingFactorial<BigInt> {
    pub fn new_big() -> Self {
        Self {
            val: BigInt::one(),
            factorials: Factorial::new_big().skip(2),
        }
    }
}

impl<T: Clone + CheckedAdd + CheckedMul + Integer> Iterator for AlternatingFactorial<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.factorials.next()? - self.val.clone();
        Some(out)
    }
}

/// The double factorial numbers. The sum of the products of the odd naturals.
///
/// ```text
/// 1, 1, 3, 15, 105, 945, 10395, 135135, 2027025, 34459425, 654729075...
/// ```
pub struct DoubleFactorial<T> {
    val: T,
    ctr: T,
}

impl DoubleFactorial<Number> {
    pub fn new() -> Self {
        Self { val: 1, ctr: 1 }
    }
}

#[cfg(feature = "big_int")]
impl DoubleFactorial<BigInt> {
    pub fn new_big() -> Self {
        Self {
            val: BigInt::one(),
            ctr: BigInt::one(),
        }
    }
}

impl<T: Clone + CheckedAdd + CheckedMul + Integer> Iterator for DoubleFactorial<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.val.checked_mul(&self.ctr)?;
        self.ctr.incr()?;
        self.ctr.incr()?;
        Some(out)
    }
}

crate::check_iteration_times!(
    Factorial::new_big(), 1_000;
);

crate::check_sequences!(
    Factorial::new_big(), [1_u128, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800, 39916800, 479001600, 6227020800, 87178291200, 1307674368000, 20922789888000, 355687428096000, 6402373705728000, 121645100408832000, 2432902008176640000, 51090942171709440000, 1124000727777607680000];
    Factorial::new(), [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
    DoubleFactorial::new_big(), [1, 1, 3, 15, 105, 945, 10395, 135135, 2027025, 34459425];
    DoubleFactorial::new(), [1, 1, 3, 15, 105, 945, 10395, 135135, 2027025, 34459425];
    AlternatingFactorial::new_big(), [1, 1, 5, 19, 101, 619, 4421, 35899, 326981, 3301819];
    AlternatingFactorial::new(), [1, 1, 5, 19, 101, 619, 4421, 35899, 326981, 3301819];
);

crate::sample_sequences!(
    Factorial::new();
    DoubleFactorial::new();
    AlternatingFactorial::new();
);
