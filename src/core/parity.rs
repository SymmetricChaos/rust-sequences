use crate::Number;
use num::{BigInt, CheckedAdd, One, Signed, Zero};
use std::marker::PhantomData;

/// The sequence of parity of the natural numbers with 0 for even and 1 for odd.
///
/// ```text
/// 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0...
/// ```
pub struct Parity<T> {
    value: bool,
    _phantom: PhantomData<T>,
}

impl Parity<Number> {
    pub fn new() -> Self {
        Self {
            value: false,
            _phantom: PhantomData,
        }
    }
}

#[cfg(feature = "big_int")]
impl Parity<BigInt> {
    pub fn new_big() -> Self {
        Self {
            value: false,
            _phantom: PhantomData,
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
///
/// ```text
/// 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34...
/// ```
pub struct Evens<T> {
    val: T,
}

impl Evens<Number> {
    pub fn new() -> Self {
        Self { val: 0 }
    }
}

#[cfg(feature = "big_int")]
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
///
/// ```text
/// 1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35...
/// ```
pub struct Odds<T> {
    val: T,
}

impl Odds<Number> {
    pub fn new() -> Self {
        Self { val: 1 }
    }
}

#[cfg(feature = "big_int")]
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
///
/// ```text
/// 0, 2, -2, 4, -4, 6, -6, 8, -8, 10, -10, 12, -12, 14, -14, 16, -16...
/// ```
pub struct EvenIntegers<T> {
    val: T,
}

impl EvenIntegers<Number> {
    pub fn new() -> Self {
        Self { val: 0 }
    }
}

#[cfg(feature = "big_int")]
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
///
/// ```text
/// 1, -1, 3, -3, 5, -5, 7, -7, 9, -9, 11, -11, 13, -13, 15, -15, 17...
/// ```
pub struct OddIntegers<T> {
    val: T,
}

impl OddIntegers<Number> {
    pub fn new() -> Self {
        Self { val: 1 }
    }
}

#[cfg(feature = "big_int")]
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
    Evens::new(), 5_000_000;
    Evens::new(), 5_000_000;
    EvenIntegers::new(), 5_000_000;
    EvenIntegers::new(), 5_000_000;
);

crate::check_sequences!(
    Evens::new(), [0, 2, 4, 6, 8, 10, 12, 14, 16, 18];
    EvenIntegers::new(), [0, 2, -2, 4, -4, 6, -6, 8, -8, 10];
    Odds::new(), [1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
    OddIntegers::new(), [1, -1, 3, -3, 5, -5, 7, -7, 9, -9];
);

crate::sample_sequences!(
    Parity::new();
    Evens::new();
    EvenIntegers::new();
    Odds::new();
    OddIntegers::new();
);
