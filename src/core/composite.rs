use num::{BigInt, One, Signed, Zero};
use std::collections::HashMap; // Found to be much faster than BTreeMap

/// The composite numbers.
/// 4, 6, 8, 9, 10, 12, 14, 15, 16...
pub struct Composite {
    sieve: HashMap<BigInt, Vec<BigInt>>,
    n: BigInt,
}

impl Composite {
    pub fn new() -> Self {
        Self {
            sieve: HashMap::<BigInt, Vec<BigInt>>::new(),
            n: BigInt::from(1),
        }
    }

    /// Includes 0 and 1, which are not composite, forming the non-prime numbers and zero.
    /// 0, 1, 4, 6, 8, 9, 10, 12, 14, 15...
    pub fn with_zero() -> Self {
        Self {
            sieve: HashMap::<BigInt, Vec<BigInt>>::new(),
            n: BigInt::from(-1),
        }
    }

    /// Includes 1, which is not composite, forming the non-prime numbers.
    /// 1, 4, 6, 8, 9, 10, 12, 14, 15, 16...
    pub fn with_one() -> Self {
        Self {
            sieve: HashMap::<BigInt, Vec<BigInt>>::new(),
            n: BigInt::from(0),
        }
    }
}

impl Iterator for Composite {
    type Item = BigInt;

    fn next(&mut self) -> Option<BigInt> {
        if self.n.is_negative() {
            self.n += 1;
            return Some(BigInt::zero());
        } else if self.n.is_zero() {
            self.n += 1;
            return Some(BigInt::one());
        } else {
            loop {
                self.n += 1;
                if !self.sieve.contains_key(&self.n) {
                    self.sieve.insert(&self.n + &self.n, vec![self.n.clone()]);
                } else {
                    let factors = &self.sieve[&self.n].clone();
                    for factor in factors {
                        if self.sieve.contains_key(&(factor + &self.n)) {
                            self.sieve
                                .get_mut(&(factor + &self.n))
                                .unwrap()
                                .push(factor.clone());
                        } else {
                            self.sieve.insert(factor + &self.n, vec![factor.clone()]);
                        }
                    }
                    self.sieve.remove(&self.n);
                    return Some(self.n.clone());
                }
            }
        }
    }
}

crate::check_sequences!(
    Composite::new(), 0, 10, [4, 6, 8, 9, 10, 12, 14, 15, 16, 18];
    Composite::with_one(), 0, 10, [1, 4, 6, 8, 9, 10, 12, 14, 15, 16];
    Composite::with_zero(), 0, 10, [0, 1, 4, 6, 8, 9, 10, 12, 14, 15];
);
