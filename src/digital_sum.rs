use crate::{Number, core::traits::Increment};
use num::{BigInt, CheckedAdd, FromPrimitive, Integer, Zero};

// Doesn't check for sign so only use internally
fn digital_sum<N: Integer>(mut n: N, base: &N) -> N {
    let mut total = N::zero();

    while n > N::zero() {
        let (div, rem) = n.div_rem(&base);
        total = total + rem;
        n = div;
    }

    total
}

// Doesn't check for sign so only use internally
fn digital_root<N: Integer>(mut n: N, base: &N) -> N {
    while n >= *base {
        n = digital_sum(n, &base)
    }
    n
}

// Doesn't check for sign so only use internally
fn additive_persistence<N: Integer>(mut n: N, base: &N) -> N {
    let mut ctr = N::zero();
    while n >= *base {
        n = digital_sum(n, &base);
        ctr = ctr + N::one();
    }
    ctr
}

/// The sums of the digits of each natural number in a given base.
///
/// ```text
/// base = 10
/// 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 2, 3...
/// ```
pub struct DigitalSums<T> {
    ctr: T,
    base: T,
}

impl DigitalSums<Number> {
    /// Base must be greater than or equal to 2.
    pub fn new(base: Number) -> Self {
        assert!(base >= 2);
        Self { ctr: 0, base }
    }
}

#[cfg(feature = "big_int")]
impl DigitalSums<BigInt> {
    /// Base must be greater than or equal to 2.
    pub fn new_big<G>(base: G) -> Self
    where
        BigInt: From<G>,
    {
        let base = BigInt::from(base);
        assert!(base >= BigInt::from_i32(2).unwrap());
        Self {
            base,
            ctr: BigInt::zero(),
        }
    }
}

impl<T: Clone + CheckedAdd + Integer> Iterator for DigitalSums<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = digital_sum(self.ctr.clone(), &self.base);
        self.ctr.incr()?;
        Some(out)
    }
}

/// The digital root of each natural number in a given base. The fixed point when repeatedly applying the digital sum.
///
/// ```text
/// base = 10
/// 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4...
/// ```
pub struct DigitalRoots<N> {
    ctr: N,
    base: N,
}

impl DigitalRoots<Number> {
    /// Base must be greater than or equal to 2.
    pub fn new(base: Number) -> Self {
        assert!(base >= 2);
        Self { ctr: 0, base }
    }
}

#[cfg(feature = "big_int")]
impl DigitalRoots<BigInt> {
    /// Base must be greater than or equal to 2.
    pub fn new_big<G>(base: G) -> Self
    where
        BigInt: From<G>,
    {
        let base = BigInt::from(base);
        assert!(base >= BigInt::from_i32(2).unwrap());
        Self {
            base,
            ctr: BigInt::zero(),
        }
    }
}

impl<T: Clone + CheckedAdd + Integer> Iterator for DigitalRoots<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = digital_root(self.ctr.clone(), &self.base);
        self.ctr.incr()?;
        Some(out)
    }
}

/// The additive persistence of each natural number in a given base. The number of times the digital sum function must be applied before it reaches a fixed point.
///
/// ```text
/// base = 10
/// 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1...
/// ```
pub struct AdditivePersistence<N> {
    ctr: N,
    base: N,
}

impl AdditivePersistence<Number> {
    /// Base must be greater than or equal to 2.
    pub fn new(base: Number) -> Self {
        assert!(base >= 2);
        Self { ctr: 0, base }
    }
}

#[cfg(feature = "big_int")]
impl AdditivePersistence<BigInt> {
    /// Base must be greater than or equal to 2.
    pub fn new_big<G>(base: G) -> Self
    where
        BigInt: From<G>,
    {
        let base = BigInt::from(base);
        assert!(base >= BigInt::from_i32(2).unwrap());
        Self {
            base,
            ctr: BigInt::zero(),
        }
    }
}

impl<T: Clone + CheckedAdd + Integer> Iterator for AdditivePersistence<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = additive_persistence(self.ctr.clone(), &self.base);
        self.ctr.incr()?;
        Some(out)
    }
}

crate::check_sequences!(
    DigitalSums::new(10), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 8, 9, 10, 11, 12, 13, 14, 15];
    DigitalRoots::new(10), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8];
    AdditivePersistence::new(10), skip 1234, [2,  2,  2,  2,   2,  2, 1, 1, 1,  2,  2,  2,  2,  2,  2,  2, 1, 1,  2,  2];
);

crate::sample_sequences!(
    DigitalSums::new(10);
    DigitalRoots::new(10);
    AdditivePersistence::new(10);
);
