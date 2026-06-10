use crate::{Number, core::traits::Increment};
use num::{BigInt, CheckedAdd, Integer, One, Zero, integer::gcd};
use std::collections::BTreeSet;

/// The ECG sequence. Starting with 1 and 2 each term is the smallest positive integer not yet used which shares a divisor with the previous term.
///
/// ```text
/// 1, 2, 4, 6, 3, 9, 12, 8, 10, 5, 15, 18, 14, 7, 21, 24, 16, 20, 22...
/// ```
pub struct Ecg<T> {
    used: BTreeSet<T>, // Checking this becomes much faster than checking a Vec after a few hundred terms
    last: T,
    initial_ctr: T,
}

impl Ecg<Number> {
    pub fn new() -> Self {
        Self {
            used: BTreeSet::from([1, 2]),
            last: 0,
            initial_ctr: 1,
        }
    }
}

#[cfg(feature = "big_int")]
impl Ecg<BigInt> {
    pub fn new_big() -> Self {
        Self {
            used: BTreeSet::from([BigInt::one(), BigInt::from(2)]),
            last: BigInt::zero(),
            initial_ctr: BigInt::one(),
        }
    }
}

impl<T: Clone + CheckedAdd + Integer> Iterator for Ecg<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.last.is_zero() {
            self.last = T::one();
            return Some(T::one());
        }
        if self.last.is_one() {
            self.last = T::one() + T::one();
            return Some(T::one() + T::one());
        }

        // Trim the btree and advance the initial counter to repeating unnecessary work
        // This slightly slows down the iterator for the first few hundred values but hugely speeds it up after the first 1000
        if self.used.contains(&self.initial_ctr) {
            loop {
                if self
                    .used
                    .contains(&(self.initial_ctr.checked_add(&T::one())?))
                {
                    self.used.remove(&self.initial_ctr);
                    self.initial_ctr.incr()?;
                } else {
                    break;
                }
            }
        }

        let mut ctr = self.initial_ctr.clone();

        loop {
            if !self.used.contains(&ctr) {
                if !gcd(ctr.clone(), self.last.clone()).is_one() {
                    self.used.insert(ctr.clone());
                    self.last = ctr.clone();
                    return Some(ctr.clone());
                }
            }
            ctr.incr()?;
        }
    }
}

crate::check_iteration_times!(
    Ecg::new(), [100, 1_000, 10_000];
);

crate::check_sequences!(
    Ecg::new(), [1, 2, 4, 6, 3, 9, 12, 8, 10, 5, 15, 18, 14, 7, 21, 24, 16, 20, 22, 11, 33, 27, 30, 25, 35, 28, 26, 13, 39, 36, 32, 34, 17, 51, 42, 38, 19, 57, 45, 40, 44, 46, 23, 69, 48, 50, 52, 54, 56, 49, 63, 60, 55, 65, 70, 58, 29, 87, 66, 62, 31, 93, 72, 64, 68, 74, 37, 111, 75, 78, 76, 80, 82];
);

crate::sample_sequences!(
    Ecg::new();
);
