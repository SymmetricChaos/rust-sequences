use crate::{Number, core::traits::Increment};
use num::{BigInt, Integer, One, Zero};

/// The evil numbers, those natural numbers having an even number of bits 1s in their binary representation. So called as a pun on its complementary sequence the odius numbers.
///
/// ```text
/// 0, 3, 5, 6, 9, 10, 12, 15, 17, 18, 20, 23, 24, 27, 29, 30, 33, 34...
/// ```
pub struct Evil<T> {
    ctr: T,
}

impl Evil<Number> {
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

#[cfg(feature = "big_int")]
impl Evil<BigInt> {
    pub fn new_big() -> Self {
        Self {
            ctr: BigInt::zero(),
        }
    }
}

impl Iterator for Evil<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.ctr.count_ones().is_even() {
                let out = self.ctr;
                self.ctr.incr()?;
                return Some(out);
            }
            self.ctr.incr()?;
        }
    }
}

#[cfg(feature = "big_int")]
impl Iterator for Evil<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.ctr.magnitude().count_ones().is_even() {
                let out = self.ctr.clone();
                self.ctr.inc();
                return Some(out);
            }
            self.ctr.inc();
        }
    }
}

/// The odious numbers, those natural numbers having an odd number of 1s in their binary representation. So called as a pun on its complementary sequence the evil numbers.
///
/// ```text
/// 1, 2, 4, 7, 8, 11, 13, 14, 16, 19, 21, 22, 25, 26, 28, 31, 32, 35...
/// ```
pub struct Odious<T> {
    ctr: T,
}

impl Odious<Number> {
    pub fn new() -> Self {
        Self { ctr: 1 }
    }
}

#[cfg(feature = "big_int")]
impl Odious<BigInt> {
    pub fn new_big() -> Self {
        Self { ctr: BigInt::one() }
    }
}

impl Iterator for Odious<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.ctr.count_ones().is_odd() {
                let out = self.ctr.clone();
                self.ctr.incr()?;
                return Some(out);
            }
            self.ctr.incr()?;
        }
    }
}

#[cfg(feature = "big_int")]
impl Iterator for Odious<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.ctr.magnitude().count_ones().is_odd() {
                let out = self.ctr.clone();
                self.ctr.inc();
                return Some(out);
            }
            self.ctr.inc();
        }
    }
}

crate::check_iteration_times!(
    Evil::new_big(), 1_000_000;
    Odious::new_big(), 1_000_000;
);

crate::check_sequences!(
    Evil::new_big(), [0, 3, 5, 6, 9, 10, 12, 15, 17, 18, 20, 23, 24, 27, 29, 30, 33, 34, 36, 39, 40, 43, 45, 46, 48, 51, 53, 54, 57, 58, 60, 63, 65, 66, 68, 71, 72, 75, 77, 78, 80, 83, 85, 86, 89, 90, 92, 95, 96, 99, 101, 102, 105, 106, 108, 111, 113, 114, 116, 119, 120, 123, 125, 126, 129];
    Odious::new_big(), [1, 2, 4, 7, 8, 11, 13, 14, 16, 19, 21, 22, 25, 26, 28, 31, 32, 35, 37, 38, 41, 42, 44, 47, 49, 50, 52, 55, 56, 59, 61, 62, 64, 67, 69, 70, 73, 74, 76, 79, 81, 82, 84, 87, 88, 91, 93, 94, 97, 98, 100, 103, 104, 107, 109, 110, 112, 115, 117, 118, 121, 122, 124, 127, 128];
);

crate::sample_sequences!(
    Evil::new();
    Odious::new();
);
