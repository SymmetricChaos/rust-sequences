use num::{BigInt, Integer, PrimInt, Signed, ToPrimitive, Zero};

pub fn digital_sum<N: Integer>(mut n: N, base: N) -> N {
    let mut total = N::zero();

    while n > N::zero() {
        let (div, rem) = n.div_rem(&base);
        total = total + rem;
        n = div;
    }

    total
}

pub fn digital_root<N: Integer + PrimInt>(mut n: N, base: N) -> N {
    while n >= base {
        n = digital_sum(n, base)
    }
    n
}

pub fn digital_sum_big(mut n: BigInt, base: u64) -> BigInt {
    let mut total = BigInt::zero();
    let base = BigInt::from(base);
    while n.is_positive() {
        let (div, rem) = n.div_rem(&base);
        total = total + rem.to_u64().unwrap(); // always works because base is limited to u64
        n = div;
    }

    total
}

pub fn digital_root_big(mut n: BigInt, base: u64) -> BigInt {
    while n >= base.into() {
        n = digital_sum_big(n, base)
    }
    n
}

/// The sums of the digits of each natural number.
pub struct DigitalSums<N> {
    ctr: N,
    base: N,
}

impl<N: PrimInt + Integer> DigitalSums<N> {
    pub fn new(base: N) -> Self {
        assert!(base >= N::from(2).unwrap());
        Self {
            ctr: N::zero(),
            base: base,
        }
    }
}

impl<N: PrimInt + Integer> Iterator for DigitalSums<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        let out = digital_sum(self.ctr.clone(), self.base);
        self.ctr = self.ctr.checked_add(&N::one())?;
        Some(out)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.ctr = self.ctr.checked_add(&N::from(n)?)?;
        let out = digital_sum(self.ctr.clone(), self.base);
        self.ctr = self.ctr.checked_add(&N::one())?;
        Some(out)
    }
}

/// The digital root of each natural number.
pub struct DigitalRoots<N> {
    ctr: N,
    base: N,
}

impl<N: PrimInt> DigitalRoots<N> {
    pub fn new(base: N) -> Self {
        assert!(base >= N::from(2).unwrap());
        Self {
            ctr: N::zero(),
            base: base,
        }
    }
}

impl<N: PrimInt + Integer> Iterator for DigitalRoots<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        let out = digital_root(self.ctr.clone(), self.base);
        self.ctr = self.ctr.checked_add(&N::one())?;
        Some(out)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.ctr = self.ctr.checked_add(&N::from(n)?)?;
        let out = digital_root(self.ctr.clone(), self.base);
        self.ctr = self.ctr.checked_add(&N::one())?;
        Some(out)
    }
}

crate::check_sequences!(
    DigitalSums::new(10), 1234, 20,  [10, 11, 12, 13, 14, 15, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 8, 9, 10, 11];
    DigitalRoots::new(10), 1234, 20, [1,  2,  3,  4,  5,  6,  7, 8, 9,  1,  2,  3,  4,  5,  6,  7, 8, 9,  1,  2];
);
