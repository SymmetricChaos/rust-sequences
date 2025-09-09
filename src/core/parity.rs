use std::marker::PhantomData;

use num::{BigInt, One, PrimInt, Signed, Zero};

/// The sequence of parity of the natural numbers with 0 for even and 1 for odd.
/// 0, 1, 0, 1, 0, 1, 0, 1, 0, 1...
pub struct Parity {
    val: bool,
}

impl Parity {
    pub fn new() -> Self {
        Self { val: false }
    }
}

impl Iterator for Parity {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        self.val = !self.val;
        if self.val {
            Some(BigInt::zero())
        } else {
            Some(BigInt::one())
        }
    }
}

/// The sequence of parity of the natural numbers with 0 for even and 1 for odd.
/// 0, 1, 0, 1, 0, 1, 0, 1, 0, 1...
pub struct ParityGeneric<T> {
    val: bool,
    _type: PhantomData<T>,
}

impl<T: PrimInt> ParityGeneric<T> {
    pub fn new() -> Self {
        Self {
            val: false,
            _type: PhantomData,
        }
    }
}

impl<T: PrimInt> Iterator for ParityGeneric<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.val = !self.val;
        if self.val {
            Some(T::zero())
        } else {
            Some(T::one())
        }
    }
}

/// The even natural numbers.
/// 0, 2, 4, 6, 8, 10, 12, 14, 16, 18...
pub struct Even {
    val: BigInt,
}

impl Even {
    pub fn new() -> Self {
        Self {
            val: BigInt::from(0),
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

    // Nearly constant time optimization for .skip()
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.val += BigInt::from(n) * 2;
        Some(self.val.clone())
    }
}

/// The odd natural numbers.
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

    // Nearly constant time optimization for .skip()
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.val += 2 * BigInt::from(n);
        Some(self.val.clone())
    }
}

/// The even integers.
/// 0, 2, -2, 4, -4, 6, -6, 8, -8, 10...
pub struct EvenInteger {
    val: BigInt,
}

impl EvenInteger {
    pub fn new() -> Self {
        Self {
            val: BigInt::from(0),
        }
    }
}

impl Iterator for EvenInteger {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();

        if self.val.is_positive() {
            self.val *= BigInt::from(-1);
        } else {
            self.val *= BigInt::from(-1);
            self.val += 2;
        };

        Some(out)
    }
}

/// The odd integers.
/// 1, -1, 3, -3, 5, -5, 7, -7, 9, -9...
pub struct OddInteger {
    val: BigInt,
}

impl OddInteger {
    pub fn new() -> Self {
        Self {
            val: BigInt::from(1),
        }
    }
}

impl Iterator for OddInteger {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();

        if self.val.is_positive() {
            self.val *= BigInt::from(-1);
        } else {
            self.val *= BigInt::from(-1);
            self.val += 2;
        };

        Some(out)
    }
}

crate::check_iteration_times!(
    Even::new(), 1_000_000;
    Even::new().skip(1_000_000), 1;
);

crate::check_sequences!(
    Even::new(), 0, 10, [0, 2, 4, 6, 8, 10, 12, 14, 16, 18];
    EvenInteger::new(), 0, 10, [0, 2, -2, 4, -4, 6, -6, 8, -8, 10];
    Odd::new(), 0, 10, [1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
    OddInteger::new(), 0, 10, [1, -1, 3, -3, 5, -5, 7, -7, 9, -9];
);
