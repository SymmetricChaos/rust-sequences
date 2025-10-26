use crate::core::prime::Primes;
use itertools::Itertools;
use num::{BigInt, CheckedAdd, CheckedDiv, Integer, Signed, Zero};
use std::hash::Hash;

/// The smooth numbers, those natural numbers for which the only prime divisors are less than or equal to n.
pub struct Smooth<T> {
    ctr: T,
    primes: Vec<T>,
}

impl<T: CheckedAdd + CheckedDiv + Clone + Hash + Integer> Smooth<T> {
    /// Panics if n is less than two.
    /// If n is very large initializing the set of primes may impose an extreme time and memory burden. There are more than two hundred million primes less than u32::MAX.
    pub fn new(n: T) -> Self {
        assert!(n > T::one());
        Self {
            ctr: T::zero(),
            primes: Primes::new().take_while(|x| *x <= n).collect_vec(),
        }
    }
}

impl Smooth<BigInt> {
    /// Panics if n is less than two.
    /// If n is very large initializing the set of primes may impose an extreme time and memory burden. There are more than two hundred million primes less than u32::MAX.
    pub fn new_big<N>(n: N) -> Self
    where
        BigInt: From<N>,
    {
        let n = BigInt::from(n);
        assert!(n.is_positive());
        Self {
            ctr: BigInt::zero(),
            primes: Primes::new_big().take_while(|x| *x <= n).collect_vec(),
        }
    }
}

impl<T: CheckedAdd + CheckedDiv + Clone + Integer> Iterator for Smooth<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.ctr = self.ctr.checked_add(&T::one())?;
            let mut n = self.ctr.clone();
            for p in self.primes.iter() {
                while n.is_multiple_of(p) {
                    n = n.checked_div(&p)?; // this always succeeds but is the only way I could find to allow division without cloning p
                }
            }
            if n.is_one() {
                return Some(self.ctr.clone());
            }
        }
    }
}

// TODO: Optimized methods exist for generating the regular aka 5-Smooth numbers

crate::check_iteration_times!(
    Smooth::new_big(5), 500;
);

crate::check_sequences!(
    Smooth::new_big(5), 9, 10, [12, 15, 16, 18, 20, 24, 25, 27, 30, 32];
);
