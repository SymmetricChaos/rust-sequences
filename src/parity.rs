use num::{BigInt, One, Signed, Zero};

/// The even natural numbers.
/// 0, 2, 4, 6, 8, 10, 12, 14, 16, 18...
pub struct Even {
    val: BigInt,
}

impl Even {
    pub fn new() -> Self {
        Self {
            val: BigInt::zero(),
        }
    }
}

impl Iterator for Even {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val += 2;
        Some(out)
    }
}

/// The even natural numbers.
/// 1, 3, 5, 7, 9, 11, 13, 15, 17, 19...
pub struct Odd {
    val: BigInt,
}

impl Odd {
    pub fn new() -> Self {
        Self { val: BigInt::one() }
    }
}

impl Iterator for Odd {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val += 2;
        Some(out)
    }
}

/// The even integers.
/// 0, 2, -2, 4, -4, 6, -6, 8, -8, 10...
pub struct EvenInteger {
    val: BigInt,
    ctr: BigInt,
}

impl EvenInteger {
    pub fn new() -> Self {
        Self {
            val: BigInt::zero(),
            ctr: BigInt::from(2),
        }
    }
}

impl Iterator for EvenInteger {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();

        if self.val.is_positive() {
            self.val -= &self.ctr;
        } else {
            self.val += &self.ctr;
        };
        self.ctr += 2;
        Some(out)
    }
}

/// The odd integers.
/// 1, -1, 3, -3, 5, -5, 7, -7, 9, -9...
pub struct OddInteger {
    val: BigInt,
    ctr: BigInt,
}

impl OddInteger {
    pub fn new() -> Self {
        Self {
            val: BigInt::one(),
            ctr: BigInt::from(2),
        }
    }
}

impl Iterator for OddInteger {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();

        if self.val.is_positive() {
            self.val -= &self.ctr;
        } else {
            self.val += &self.ctr;
        };
        self.ctr += 2;
        Some(out)
    }
}

crate::print_a_few!(
    Even::new(), 0, 10;
    EvenInteger::new(), 0, 10;
    Odd::new(), 0, 10;
    OddInteger::new(), 0, 10;
);
