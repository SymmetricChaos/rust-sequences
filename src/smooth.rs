use crate::core::Primes;
use itertools::Itertools;
use num::{BigInt, Integer, One, Signed, Zero};

/// The smooth numbers, those natural numbers for which the only prime divisors are less than or equal to n.
pub struct Smooth {
    ctr: BigInt,
    primes: Vec<BigInt>,
}

impl Smooth {
    /// Panics if n is less than two.
    /// If n is very large initializing the set of primes may impose an extreme time and memory burden. There are more than two hundred million primes less than u32::MAX.
    pub fn new_big<T>(n: T) -> Self
    where
        BigInt: From<T>,
    {
        let n = BigInt::from(n);
        assert!(n.is_positive());
        Self {
            ctr: BigInt::zero(),
            primes: Primes::new_big().take_while(|x| *x <= n).collect_vec(),
        }
    }
}

impl Iterator for Smooth {
    type Item = BigInt;

    fn next(&mut self) -> Option<BigInt> {
        loop {
            self.ctr += 1;
            let mut n = self.ctr.clone();
            for p in self.primes.iter() {
                while n.is_multiple_of(p) {
                    n = n / p
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
