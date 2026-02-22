use num::{BigInt, CheckedAdd, Integer, One, Zero};

// Doesn't check for sign so only use internally
fn digital_prod<N: Integer>(mut n: N, base: &N) -> N {
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

/// The product of the digits of each natural number.
pub struct DigitalProds<N> {
    ctr: N,
    base: N,
}

impl<N: CheckedAdd + Clone + Integer> DigitalProds<N> {
    pub fn new(base: N) -> Self {
        assert!(base >= N::one() + N::one());
        Self {
            ctr: N::zero(),
            base: base,
        }
    }
}

impl DigitalProds<BigInt> {
    pub fn new_big<G>(base: G) -> Self
    where
        BigInt: From<G>,
    {
        let base = BigInt::from(base);
        assert!(base >= BigInt::one() + BigInt::one());
        Self {
            ctr: BigInt::zero(),
            base,
        }
    }
}

impl<N: CheckedAdd + Clone + Integer> Iterator for DigitalProds<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        let out = digital_prod(self.ctr.clone(), &self.base);
        self.ctr = self.ctr.checked_add(&N::one())?;
        Some(out)
    }
}

/// The multiplicative digital root of each natural number. The fixed point when repeatedly applying the digital product.
pub struct MultiplicativeDigitalRoots<N> {
    ctr: N,
    base: N,
}

impl<N: CheckedAdd + Clone + Integer> MultiplicativeDigitalRoots<N> {
    pub fn new(base: N) -> Self {
        assert!(base >= N::one() + N::one());
        Self {
            ctr: N::zero(),
            base: base,
        }
    }
}
impl MultiplicativeDigitalRoots<BigInt> {
    pub fn new_big<G>(base: G) -> Self
    where
        BigInt: From<G>,
    {
        let b = BigInt::from(base);
        assert!(b >= BigInt::one() + BigInt::one());
        Self {
            ctr: BigInt::zero(),
            base: b,
        }
    }
}

impl<N: CheckedAdd + Clone + Integer> Iterator for MultiplicativeDigitalRoots<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        let out = multiplicative_digital_root(self.ctr.clone(), &self.base);
        self.ctr = self.ctr.checked_add(&N::one())?;
        Some(out)
    }
}

/// The multiplicative persistence of each natural number. The number of times the digital sum function must be applied before it reaches a fixed point.
pub struct MultiplicativePersistence<N> {
    ctr: N,
    base: N,
}

impl<N: CheckedAdd + Clone + Integer> MultiplicativePersistence<N> {
    pub fn new(base: N) -> Self {
        assert!(base >= N::one() + N::one());
        Self {
            ctr: N::zero(),
            base: base,
        }
    }
}

impl<N: CheckedAdd + Clone + Integer> Iterator for MultiplicativePersistence<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        let out = multiplicative_persistence(self.ctr.clone(), &self.base);
        self.ctr = self.ctr.checked_add(&N::one())?;
        Some(out)
    }
}

crate::check_sequences!(
    DigitalProds::new(10), 25,               [10, 12, 14, 16, 18, 0, 3, 6, 9, 12];
    MultiplicativeDigitalRoots::new(10), 25, [0,   2,  4,  6,  8, 0, 3, 6, 9,  2];
    MultiplicativePersistence::new(10), 25,  [2,   2,  2,  2,  2, 1, 1, 1, 1,  2];
);
