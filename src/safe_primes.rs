use crate::core::primes::Primes;
use num::{BigInt, CheckedAdd, Integer};
use std::hash::Hash;

pub struct SafePrimes<T> {
    primes: Primes<T>,
    list: Vec<T>,
}

impl<T: CheckedAdd + Clone + Hash + Integer> SafePrimes<T> {
    pub fn new() -> Self {
        let mut primes = Primes::new();
        let list = vec![primes.next().unwrap()];
        SafePrimes { primes, list }
    }
}

impl SafePrimes<BigInt> {
    pub fn new_bigs() -> Self {
        SafePrimes::new()
    }
}

impl<T: CheckedAdd + Clone + Hash + Integer> Iterator for SafePrimes<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let p = self.primes.next()?;
            self.list.push(p.clone());
            let c = (p.clone() - T::one()) / (T::one() + T::one());
            if self.list.contains(&c) {
                return Some(p);
            }
        }
    }
}

pub struct SophieGermainPrimes<T> {
    primes: Primes<T>,
    list: Vec<T>,
}

impl<T: CheckedAdd + Clone + Hash + Integer> SophieGermainPrimes<T> {
    pub fn new() -> Self {
        let mut primes = Primes::new();
        let list = vec![primes.next().unwrap()];
        SophieGermainPrimes { primes, list }
    }
}

impl SophieGermainPrimes<BigInt> {
    pub fn new_bigs() -> Self {
        SophieGermainPrimes::new()
    }
}

impl<T: CheckedAdd + Clone + Hash + Integer> Iterator for SophieGermainPrimes<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let p = self.primes.next()?;
            self.list.push(p.clone());
            let c = (p.clone() - T::one()) / (T::one() + T::one());
            if self.list.contains(&c) {
                return Some(c);
            }
        }
    }
}

crate::check_sequences!(
    SafePrimes::<i32>::new(), [5, 7, 11, 23, 47, 59, 83, 107, 167, 179, 227, 263, 347, 359, 383, 467, 479, 503, 563, 587, 719, 839, 863, 887, 983, 1019, 1187, 1283, 1307, 1319, 1367, 1439, 1487, 1523, 1619, 1823, 1907];
    SophieGermainPrimes::<i32>::new(), [2, 3, 5, 11, 23, 29, 41, 53, 83, 89, 113, 131, 173, 179, 191, 233, 239, 251, 281, 293, 359, 419, 431, 443, 491, 509, 593, 641, 653, 659, 683, 719, 743, 761, 809, 911, 953];
);
