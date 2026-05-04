use crate::core::traits::Increment;
use num::{BigInt, CheckedAdd, Integer};
use std::collections::BTreeSet;

/// The anti-Fibonacci numbers. Starting with zero the next term is the sum of the two smallest unique positive integers that have not previously been added together or been a sum.
///
/// 0, 3, 9, 13, 18, 23, 29, 33, 39, 43...
pub struct AntiFibonacci<T> {
    terms: BTreeSet<T>,
    a: T,
    b: T,
}

impl<T: CheckedAdd + Clone + Integer> AntiFibonacci<T> {
    pub fn new() -> Self {
        Self {
            terms: BTreeSet::new(),
            a: T::zero(),
            b: T::zero(),
        }
    }
}

impl AntiFibonacci<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: CheckedAdd + Clone + Integer> Iterator for AntiFibonacci<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.checked_add(&self.b)?;

        self.terms.insert(out.clone());

        while self.terms.contains(&self.a) {
            self.a = self.a.checked_add(&T::one())?;
        }
        self.terms.insert(self.a.clone());

        while self.terms.contains(&self.b) {
            self.b = self.b.checked_add(&T::one())?;
        }
        self.terms.insert(self.b.clone());

        Some(out)
    }
}

/// The non-anti-Fibonacci numbers.
///
/// 1, 2, 4, 5, 6, 7, 8, 10, 11, 12, 14, 15, 16, 17, 19, 20...
pub struct NonAntiFibonacci<T> {
    antifib: AntiFibonacci<T>,
    record: T,
    ctr: T,
}

impl<T: CheckedAdd + Clone + Integer> NonAntiFibonacci<T> {
    pub fn new() -> Self {
        let mut antifib = AntiFibonacci::new();
        antifib.next();
        let record = antifib.next().unwrap();
        Self {
            antifib,
            record,
            ctr: T::one(),
        }
    }
}

impl NonAntiFibonacci<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: CheckedAdd + Clone + Integer> Iterator for NonAntiFibonacci<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.ctr.clone();
        loop {
            self.ctr.incr()?;
            if self.ctr == self.record {
                self.record = self.antifib.next()?;
            } else {
                break;
            }
        }
        Some(out)
    }
}

crate::check_iteration_times!(
    AntiFibonacci::<i32>::new(), [100, 1_000, 10_000, 100_000, 1_000_000];
    AntiFibonacci::new_big(), [100, 1_000, 10_000, 100_000, 1_000_000];
);

crate::check_sequences!(
    AntiFibonacci::new_big(), [
        0, 3, 9, 13, 18, 23, 29, 33, 39, 43, 49, 53, 58, 63, 69,
        73, 78, 83, 89, 93, 98, 103, 109, 113, 119, 123, 129,
        133, 138, 143, 149, 153, 159, 163, 169, 173, 178, 183,
        189, 193, 199, 203, 209, 213, 218, 223, 229, 233, 238,
        243, 249, 253, 258, 263, 269, 273, 279, 283];
    NonAntiFibonacci::new_big(), [
        1, 2, 4, 5, 6, 7, 8, 10, 11, 12, 14, 15, 16, 17, 19, 20,
        21, 22, 24, 25, 26, 27, 28, 30, 31, 32, 34, 35, 36, 37,
        38, 40, 41, 42, 44, 45, 46, 47, 48, 50, 51, 52, 54, 55,
        56, 57, 59, 60, 61, 62, 64, 65, 66, 67, 68, 70, 71, 72,
        74, 75, 76, 77, 79, 80, 81, 82, 84, 85, 86, 87, 88, 90,
        91, 92, 94, 95, 96, 97, 99, 100];
);
