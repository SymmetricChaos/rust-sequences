use itertools::Itertools;
use num::{BigInt, Integer, One};

use crate::primes::Prime;

/// The regular numbers, those natural numbers for which the only prime divisors are less than n.
pub struct Smooth {
    ctr: BigInt,
    primes: Vec<BigInt>,
}

impl Smooth {
    /// Panics if n is less than two.
    pub fn new(n: i64) -> Self {
        assert!(n > 1);
        Self {
            ctr: BigInt::from(0),
            primes: Prime::new()
                .take_while(|x| *x <= BigInt::from(n))
                .collect_vec(),
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

crate::check_times!(
    Smooth::new(5), 600;
);

crate::check_sequences!(
    Smooth::new(5), 9, 10, [12, 15, 16, 18, 20, 24, 25, 27, 30, 32];
);
