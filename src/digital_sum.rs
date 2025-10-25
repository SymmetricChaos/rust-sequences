use num::{BigInt, CheckedAdd, Integer, One, Zero};

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

/// The sums of the digits of each natural number.
pub struct DigitalSums<N> {
    ctr: N,
    base: N,
}

impl<N: CheckedAdd + Clone + Integer> DigitalSums<N> {
    pub fn new(base: N) -> Self {
        assert!(base >= N::one() + N::one());
        Self {
            ctr: N::zero(),
            base: base,
        }
    }
}

impl DigitalSums<BigInt> {
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

impl<N: CheckedAdd + Clone + Integer> Iterator for DigitalSums<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        let out = digital_sum(self.ctr.clone(), &self.base);
        self.ctr = self.ctr.checked_add(&N::one())?;
        Some(out)
    }
}

/// The digital root of each natural number. The fixed point when repeatedly applying the digital sum.
pub struct DigitalRoots<N> {
    ctr: N,
    base: N,
}

impl<N: CheckedAdd + Clone + Integer> DigitalRoots<N> {
    pub fn new(base: N) -> Self {
        assert!(base >= N::one() + N::one());
        Self {
            ctr: N::zero(),
            base: base,
        }
    }
}

impl DigitalRoots<BigInt> {
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

impl<N: CheckedAdd + Clone + Integer> Iterator for DigitalRoots<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        let out = digital_root(self.ctr.clone(), &self.base);
        self.ctr = self.ctr.checked_add(&N::one())?;
        Some(out)
    }
}

/// The additive persistence of each natural number. The number of times the digital sum function must be applied before it reaches a fixed point.
pub struct AdditivePersistence<N> {
    ctr: N,
    base: N,
}

impl<N: CheckedAdd + Clone + Integer> AdditivePersistence<N> {
    pub fn new(base: N) -> Self {
        assert!(base >= N::one() + N::one());
        Self {
            ctr: N::zero(),
            base: base,
        }
    }
}

impl AdditivePersistence<BigInt> {
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

impl<N: CheckedAdd + Clone + Integer> Iterator for AdditivePersistence<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        let out = additive_persistence(self.ctr.clone(), &self.base);
        self.ctr = self.ctr.checked_add(&N::one())?;
        Some(out)
    }
}

crate::check_sequences!(
    DigitalSums::new(10), 1234, 20,         [10, 11, 12, 13, 14, 15, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 8, 9, 10, 11];
    DigitalRoots::new(10), 1234, 20,        [1,  2,  3,  4,   5,  6, 7, 8, 9,  1,  2,  3,  4,  5,  6,  7, 8, 9,  1,  2];
    AdditivePersistence::new(10), 1234, 20, [2,  2,  2,  2,   2,  2, 1, 1, 1,  2,  2,  2,  2,  2,  2,  2, 1, 1,  2,  2];
);
