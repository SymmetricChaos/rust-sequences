use crate::core::Primes;
use num::{CheckedAdd, Integer};
use std::hash::Hash;

/// The lesser of each pair of twin primes.
///
/// 3, 5, 11, 17, 29, 41, 59, 71, 101, 107, 137, 149...
pub struct TwinPrimesLesser<T> {
    primes: Primes<T>,
    prev: T,
}

impl<T: Clone + CheckedAdd + Hash + Integer> TwinPrimesLesser<T> {
    pub fn new() -> Self {
        Self {
            primes: Primes::new(),
            prev: T::one(),
        }
    }
}

impl<T: Clone + CheckedAdd + Hash + Integer> Iterator for TwinPrimesLesser<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let p = self.primes.next()?;
            if p.clone() - (T::one() + T::one()) == self.prev {
                let out = self.prev.clone();
                self.prev = p;
                return Some(out);
            } else {
                self.prev = p;
            }
        }
    }
}

/// The greater of each pair of twin primes.
///
/// 5, 7, 13, 19, 31, 43, 61, 73, 103, 109, 139, 151...
pub struct TwinPrimesGreater<T> {
    primes: Primes<T>,
    prev: T,
}

impl<T: Clone + CheckedAdd + Hash + Integer> TwinPrimesGreater<T> {
    pub fn new() -> Self {
        Self {
            primes: Primes::new(),
            prev: T::one(),
        }
    }
}

impl<T: Clone + CheckedAdd + Hash + Integer> Iterator for TwinPrimesGreater<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let p = self.primes.next()?;
            if p.clone() - (T::one() + T::one()) == self.prev {
                self.prev = p;
                return Some(self.prev.clone());
            } else {
                self.prev = p;
            }
        }
    }
}

/// Each pair of twin primes.
///
///
pub struct TwinPrimePairs<T> {
    primes: Primes<T>,
    prev: T,
}

impl<T: Clone + CheckedAdd + Hash + Integer> TwinPrimePairs<T> {
    pub fn new() -> Self {
        Self {
            primes: Primes::new(),
            prev: T::one(),
        }
    }
}

impl<T: Clone + CheckedAdd + Hash + Integer> Iterator for TwinPrimePairs<T> {
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let p = self.primes.next()?;
            if p.clone() - (T::one() + T::one()) == self.prev {
                let out = (self.prev.clone(), p.clone());
                self.prev = p;
                return Some(out);
            } else {
                self.prev = p;
            }
        }
    }
}

crate::print_sequences!(
    TwinPrimePairs::<i32>::new(), 15, "{:?}", ", ";
);

crate::check_sequences!(
    TwinPrimesLesser::<i32>::new(),  [3, 5, 11, 17, 29, 41, 59, 71, 101, 107, 137, 149, 179, 191, 197, 227, 239, 269, 281, 311, 347, 419, 431, 461, 521, 569, 599, 617, 641, 659, 809, 821, 827, 857, 881, 1019, 1031, 1049, 1061, 1091, 1151, 1229, 1277, 1289, 1301, 1319, 1427, 1451, 1481, 1487, 1607];
    TwinPrimesGreater::<i32>::new(), [5, 7, 13, 19, 31, 43, 61, 73, 103, 109, 139, 151, 181, 193, 199, 229, 241, 271, 283, 313, 349, 421, 433, 463, 523, 571, 601, 619, 643, 661, 811, 823, 829, 859, 883, 1021, 1033, 1051, 1063, 1093, 1153, 1231, 1279, 1291, 1303, 1321, 1429, 1453, 1483, 1489, 1609];
);
