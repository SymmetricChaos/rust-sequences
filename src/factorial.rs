use crate::{Number, core::traits::Increment};
use num::{BigInt, Integer, One};
use std::iter::Skip;

/// The factorial numbers. Each term is the previous term multiplied by the next positive integer. Equivalently f(n) = f(n-1) * n.
///
/// ```text
/// 1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800...
/// ```
pub struct Factorials<T> {
    val: T,
    ctr: T,
}

impl Factorials<Number> {
    pub fn new() -> Self {
        Self { val: 1, ctr: 1 }
    }
}

impl Factorials<BigInt> {
    pub fn new_big() -> Self {
        Self {
            val: BigInt::one(),
            ctr: BigInt::one(),
        }
    }
}

impl Iterator for Factorials<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.val.checked_mul(self.ctr)?;
        self.ctr.incr()?;
        Some(out)
    }
}

impl Iterator for Factorials<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.val.checked_mul(&self.ctr)?;
        self.ctr.inc();
        Some(out)
    }
}

/// The alternating factorial numbers. The partial sums of the alternating sum of the factorials. Equivalently f(n) = n! - f(n-1).
///
/// ```text
/// 1, 1, 5, 19, 101, 619, 4421, 35899, 326981...
/// ```
pub struct AlternatingFactorials<T> {
    val: T,
    factorials: Skip<Factorials<T>>,
}

impl AlternatingFactorials<Number> {
    pub fn new() -> Self {
        Self {
            val: 1,
            factorials: Factorials::new().skip(2),
        }
    }
}

impl AlternatingFactorials<BigInt> {
    pub fn new_big() -> Self {
        Self {
            val: BigInt::one(),
            factorials: Factorials::new_big().skip(2),
        }
    }
}

impl Iterator for AlternatingFactorials<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.factorials.next()? - self.val;
        Some(out)
    }
}

impl Iterator for AlternatingFactorials<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.factorials.next()? - &self.val;
        Some(out)
    }
}

/// The double factorial numbers. The sum of the products of the odd naturals.
///
/// ```text
/// 1, 1, 3, 15, 105, 945, 10395, 135135, 2027025, 34459425...
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

impl DoubleFactorial<BigInt> {
    pub fn new_big() -> Self {
        Self {
            val: BigInt::one(),
            ctr: BigInt::one(),
        }
    }
}

impl Iterator for DoubleFactorial<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.val.checked_mul(self.ctr)?;
        self.ctr = self.ctr.checked_add(2)?;
        Some(out)
    }
}

impl Iterator for DoubleFactorial<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val *= &self.ctr;
        self.ctr += 2;
        Some(out)
    }
}

crate::check_iteration_times!(
    Factorials::new_big(), 1_000;
);

crate::check_sequences!(
    Factorials::new_big(), [1_u128, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800, 39916800, 479001600, 6227020800, 87178291200, 1307674368000, 20922789888000, 355687428096000, 6402373705728000, 121645100408832000, 2432902008176640000, 51090942171709440000, 1124000727777607680000];
    Factorials::new(), [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
    DoubleFactorial::new_big(), [1, 1, 3, 15, 105, 945, 10395, 135135, 2027025, 34459425];
    DoubleFactorial::new(), [1, 1, 3, 15, 105, 945, 10395, 135135, 2027025, 34459425];
    AlternatingFactorials::new_big(), [1, 1, 5, 19, 101, 619, 4421, 35899, 326981, 3301819];
    AlternatingFactorials::new(), [1, 1, 5, 19, 101, 619, 4421, 35899, 326981, 3301819];
);
