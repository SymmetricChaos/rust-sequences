use crate::{Number, core::traits::Increment};
use num::{BigInt, CheckedAdd, FromPrimitive, Integer, Zero};

// Doesn't check for sign so only use internally
fn digital_prod<N: Integer>(mut n: N, base: &N) -> N {
    if n.is_zero() {
        return N::zero();
    }

    let mut total = N::one();

    while n > N::zero() {
        let (div, rem) = n.div_rem(&base);
        // shortcut on finding a zero digit
        if rem.is_zero() {
            return N::zero();
        }
        total = total * rem;
        n = div;
    }

    total
}

fn multiplicative_digital_root<N: Integer>(mut n: N, base: &N) -> N {
    while n >= *base {
        n = digital_prod(n, &base)
    }
    n
}

// Doesn't check for sign so only use internally
fn multiplicative_persistence<N: Integer>(mut n: N, base: &N) -> N {
    let mut ctr = N::zero();
    while n >= *base {
        n = digital_prod(n, &base);
        ctr = ctr + N::one();
    }
    ctr
}

/// The product of the digits of each natural number to a given base.
///
/// ```text
/// base = 10
/// 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 2, 4...
/// ```
pub struct DigitalProds<T> {
    ctr: T,
    base: T,
}

impl DigitalProds<Number> {
    /// Base must be greater than or equal to 2.
    pub fn new(base: Number) -> Self {
        assert!(base >= 2);
        Self { ctr: 0, base }
    }
}

impl DigitalProds<BigInt> {
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

impl<T: Clone + CheckedAdd + Integer> Iterator for DigitalProds<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = digital_prod(self.ctr.clone(), &self.base);
        self.ctr.incr()?;
        Some(out)
    }
}

/// The multiplicative digital root of each natural number to a given base. The fixed point when repeatedly applying the digital product.
///
/// ```text
/// base = 10
/// 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 2, 4...
/// ```
pub struct MultiplicativeDigitalRoots<T> {
    ctr: T,
    base: T,
}

impl MultiplicativeDigitalRoots<Number> {
    /// Base must be greater than or equal to 2.
    pub fn new(base: Number) -> Self {
        assert!(base >= 2);
        Self { ctr: 0, base }
    }
}

impl MultiplicativeDigitalRoots<BigInt> {
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

impl<T: Clone + CheckedAdd + Integer> Iterator for MultiplicativeDigitalRoots<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = multiplicative_digital_root(self.ctr.clone(), &self.base);
        self.ctr.incr()?;
        Some(out)
    }
}

/// The multiplicative persistence of each natural number to a given base. The number of times the digital product function must be applied before it reaches a fixed point.
///
/// ```text
/// base = 10
/// 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1...
/// ```
pub struct MultiplicativePersistence<T> {
    ctr: T,
    base: T,
}

impl MultiplicativePersistence<Number> {
    /// Base must be greater than or equal to 2.
    pub fn new(base: Number) -> Self {
        assert!(base >= 2);
        Self { ctr: 0, base }
    }
}

impl MultiplicativePersistence<BigInt> {
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

impl<T: Clone + CheckedAdd + Integer> Iterator for MultiplicativePersistence<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = multiplicative_persistence(self.ctr.clone(), &self.base);
        self.ctr.incr()?;
        Some(out)
    }
}

crate::check_sequences!(
    DigitalProds::new(10), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 0, 3, 6, 9, 12, 15, 18, 21, 24, 27, 0, 4, 8, 12, 16, 20, 24, 28, 32, 36, 0, 5, 10, 15, 20, 25, 30, 35, 40, 45, 0, 6, 12, 18, 24, 30, 36, 42];
    MultiplicativeDigitalRoots::new(10), skip 25, [0,   2,  4,  6,  8, 0, 3, 6, 9,  2];
    MultiplicativePersistence::new(10), skip 25,  [2,   2,  2,  2,  2, 1, 1, 1, 1,  2];
);

crate::sample_sequences!(
    DigitalProds::new(10);
    MultiplicativeDigitalRoots::new(10);
    MultiplicativePersistence::new(10);
);
