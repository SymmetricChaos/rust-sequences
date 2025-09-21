use std::marker::PhantomData;

use num::{BigInt, One, PrimInt, Signed, Zero};

/// The sequence of parity of the natural numbers with 0 for even and 1 for odd.
/// 0, 1, 0, 1, 0, 1, 0, 1, 0, 1...
pub struct Parity<T> {
    value: bool,
    _type: PhantomData<T>,
}

impl<T: PrimInt> Parity<T> {
    pub fn new() -> Self {
        Self {
            value: false,
            _type: PhantomData,
        }
    }
}

impl Parity<BigInt> {
    pub fn new_big() -> Self {
        Self {
            value: false,
            _type: PhantomData,
        }
    }
}

impl<T: One + Zero> Iterator for Parity<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.value = !self.value;
        if self.value {
            Some(T::zero())
        } else {
            Some(T::one())
        }
    }
}

/// The even natural numbers.
/// 0, 2, 4, 6, 8, 10, 12, 14, 16, 18...
pub struct Evens {
    val: BigInt,
}

impl Evens {
    pub fn new_big() -> Self {
        Self {
            val: BigInt::from(0),
        }
    }
}

impl Iterator for Evens {
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
pub struct Odds {
    val: BigInt,
}

impl Odds {
    pub fn new_big() -> Self {
        Self { val: BigInt::one() }
    }
}

impl Iterator for Odds {
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
pub struct EvenIntegers {
    val: BigInt,
}

impl EvenIntegers {
    pub fn new_big() -> Self {
        Self {
            val: BigInt::from(0),
        }
    }
}

impl Iterator for EvenIntegers {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        if self.val.is_positive() {
            self.val = -&self.val;
        } else {
            self.val = -&self.val;
            self.val += 2;
        };

        Some(out)
    }
}

/// The odd integers.
/// 1, -1, 3, -3, 5, -5, 7, -7, 9, -9...
pub struct OddIntegers {
    val: BigInt,
}

impl OddIntegers {
    pub fn new_big() -> Self {
        Self {
            val: BigInt::from(1),
        }
    }
}

impl Iterator for OddIntegers {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();

        if self.val.is_positive() {
            self.val = -&self.val;
        } else {
            self.val = -&self.val;
            self.val += 2;
        };

        Some(out)
    }
}

crate::check_iteration_times!(
    Evens::new_big(), 1_000_000;
    Evens::new_big().skip(1_000_000), 1;
);

crate::check_sequences!(
    Evens::new_big(), 0, 10, [0, 2, 4, 6, 8, 10, 12, 14, 16, 18];
    EvenIntegers::new_big(), 0, 10, [0, 2, -2, 4, -4, 6, -6, 8, -8, 10];
    Odds::new_big(), 0, 10, [1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
    OddIntegers::new_big(), 0, 10, [1, -1, 3, -3, 5, -5, 7, -7, 9, -9];
);
