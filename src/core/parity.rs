use std::marker::PhantomData;

use num::{BigInt, CheckedAdd, One, Signed, Zero};

/// The sequence of parity of the natural numbers with 0 for even and 1 for odd.
/// 0, 1, 0, 1, 0, 1, 0, 1, 0, 1...
pub struct Parity<T> {
    value: bool,
    _type: PhantomData<T>,
}

impl<T: One + Zero> Parity<T> {
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
pub struct Evens<T> {
    val: T,
}

impl<T: CheckedAdd + Clone + One + Zero> Evens<T> {
    pub fn new() -> Self {
        Self { val: T::zero() }
    }
}

impl Evens<BigInt> {
    pub fn new_big() -> Self {
        Self {
            val: BigInt::zero(),
        }
    }
}

impl<T: CheckedAdd + Clone + One> Iterator for Evens<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.val.checked_add(&(T::one() + T::one()))?;
        Some(out)
    }
}

/// The odd natural numbers.
/// 1, 3, 5, 7, 9, 11, 13, 15, 17, 19...
pub struct Odds<T> {
    val: T,
}

impl<T: CheckedAdd + Clone + One> Odds<T> {
    pub fn new() -> Self {
        Self { val: T::one() }
    }
}

impl Odds<BigInt> {
    pub fn new_big() -> Self {
        Self { val: BigInt::one() }
    }
}

impl<T: CheckedAdd + Clone + One> Iterator for Odds<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.val.checked_add(&(T::one() + T::one()))?;
        Some(out)
    }
}

/// The even integers.
/// 0, 2, -2, 4, -4, 6, -6, 8, -8, 10...
pub struct EvenIntegers<T> {
    val: T,
}

impl<T: CheckedAdd + Clone + One + Signed + Zero> EvenIntegers<T> {
    pub fn new() -> Self {
        Self { val: T::zero() }
    }
}

impl EvenIntegers<BigInt> {
    pub fn new_big() -> Self {
        Self {
            val: BigInt::zero(),
        }
    }
}

impl<T: CheckedAdd + Clone + Signed + One> Iterator for EvenIntegers<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        if self.val.is_positive() {
            self.val = -self.val.clone();
        } else {
            self.val = -self.val.clone();
            self.val = self.val.checked_add(&(T::one() + T::one()))?;
        };

        Some(out)
    }
}

/// The odd integers.
/// 1, -1, 3, -3, 5, -5, 7, -7, 9, -9...
pub struct OddIntegers<T> {
    val: T,
}

impl<T: CheckedAdd + Clone + Signed + One> OddIntegers<T> {
    pub fn new() -> Self {
        Self { val: T::one() }
    }
}

impl OddIntegers<BigInt> {
    pub fn new_big() -> Self {
        Self { val: BigInt::one() }
    }
}

impl<T: CheckedAdd + Clone + Signed + One> Iterator for OddIntegers<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();

        if self.val.is_positive() {
            self.val = -self.val.clone();
        } else {
            self.val = -self.val.clone();
            self.val = self.val.checked_add(&(T::one() + T::one()))?;
        };

        Some(out)
    }
}

crate::check_iteration_times!(
    Evens::new_big(), 5_000_000;
    Evens::<i32>::new(), 5_000_000;
    EvenIntegers::new_big(), 5_000_000;
    EvenIntegers::<i32>::new(), 5_000_000;
);

crate::check_sequences!(
    Evens::<i32>::new(), 0, 10, [0, 2, 4, 6, 8, 10, 12, 14, 16, 18];
    EvenIntegers::new_big(), 0, 10, [0, 2, -2, 4, -4, 6, -6, 8, -8, 10];
    Odds::new_big(), 0, 10, [1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
    OddIntegers::new_big(), 0, 10, [1, -1, 3, -3, 5, -5, 7, -7, 9, -9];
);
