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

pub fn digital_sum_big(mut n: BigInt, base: &BigInt) -> BigInt {
    let mut total = BigInt::zero();

    while n.is_positive() {
        let (div, rem) = n.div_rem(&base);
        total = total + rem.to_u64().unwrap(); // always works because base is limited to u64
        n = div;
    }

    total
}

pub fn digital_root_big(mut n: BigInt, base: &BigInt) -> BigInt {
    while n >= *base {
        n = digital_sum_big(n, base)
    }
    n
}

pub struct DigitalSums<N> {
    ctr: N,
    base: N,
}

impl DigitalSums<BigInt> {
    pub fn new_big(base: u64) -> Self {
        assert!(base >= 2);
        Self {
            ctr: BigInt::zero(),
            base: BigInt::from(base),
        }
    }
}

impl Iterator for DigitalSums<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = digital_sum_big(self.ctr.clone(), &self.base);
        self.ctr += 1;
        Some(out)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.ctr += n;
        let out = digital_sum_big(self.ctr.clone(), &self.base);
        self.ctr += 1;
        Some(out)
    }
}

pub struct DigitalRoots<N> {
    ctr: N,
    base: N,
}

impl DigitalRoots<BigInt> {
    pub fn new_big(base: u64) -> Self {
        assert!(base >= 2);
        Self {
            ctr: BigInt::zero(),
            base: BigInt::from(base),
        }
    }
}

impl Iterator for DigitalRoots<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = digital_root_big(self.ctr.clone(), &self.base);
        self.ctr += 1;
        Some(out)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.ctr += n;
        let out = digital_root_big(self.ctr.clone(), &self.base);
        self.ctr += 1;
        Some(out)
    }
}

crate::check_sequences!(
    DigitalSums::new_big(10), 123, 10, [6, 7, 8, 9, 10, 11, 12, 4, 5, 6];
    DigitalRoots::new_big(10), 123, 10, [6, 7, 8, 9, 1, 2, 3, 4, 5, 6];
);
