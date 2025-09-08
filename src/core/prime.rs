use num::{BigInt, One, PrimInt, Zero};
use std::{collections::HashMap, hash::Hash}; // Found to be much faster than BTreeMap

/// The prime natural numbers.
/// 2, 3, 5, 7, 11, 13, 17, 19, 23, 29...
pub struct Prime {
    sieve: HashMap<BigInt, Vec<BigInt>>,
    n: BigInt,
}

impl Prime {
    pub fn new() -> Self {
        Self {
            sieve: HashMap::<BigInt, Vec<BigInt>>::new(),
            n: BigInt::from(1),
        }
    }

    /// An older definition of primes also known as the non-composite numbers.
    /// 1, 2, 3, 5, 7, 11, 13, 17, 19, 23...
    pub fn with_one() -> Self {
        Self {
            sieve: HashMap::<BigInt, Vec<BigInt>>::new(),
            n: BigInt::from(0),
        }
    }
}

impl Iterator for Prime {
    type Item = BigInt;

    fn next(&mut self) -> Option<BigInt> {
        if self.n.is_zero() {
            self.n += 1;
            return Some(BigInt::one());
        }
        loop {
            self.n += 1;
            if !self.sieve.contains_key(&self.n) {
                self.sieve.insert(&self.n + &self.n, vec![self.n.clone()]);
                return Some(self.n.clone());
            } else {
                let factors = &self.sieve[&self.n].clone();
                for factor in factors {
                    let s = factor + &self.n;
                    if self.sieve.contains_key(&s) {
                        self.sieve.get_mut(&s).unwrap().push(factor.clone());
                    } else {
                        self.sieve.insert(s, vec![factor.clone()]);
                    }
                }
                self.sieve.remove(&self.n);
            }
        }
    }
}

/// The prime natural numbers.
/// 2, 3, 5, 7, 11, 13, 17, 19, 23, 29...
pub struct PrimeGeneric<T> {
    sieve: HashMap<T, Vec<T>>,
    n: T,
}

impl<T: PrimInt> PrimeGeneric<T> {
    pub fn new() -> Self {
        Self {
            sieve: HashMap::<T, Vec<T>>::new(),
            n: T::one(),
        }
    }

    /// An older definition of primes also known as the non-composite numbers.
    /// 1, 2, 3, 5, 7, 11, 13, 17, 19, 23...
    pub fn with_one() -> Self {
        Self {
            sieve: HashMap::<T, Vec<T>>::new(),
            n: T::zero(),
        }
    }
}

impl<T: PrimInt + Hash> Iterator for PrimeGeneric<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.n.is_zero() {
            self.n = self.n.checked_add(&T::one())?;
            return Some(T::one());
        }
        loop {
            self.n = self.n.checked_add(&T::one())?;
            if !self.sieve.contains_key(&self.n) {
                let v = self.n.checked_add(&self.n)?;
                self.sieve.insert(v, vec![self.n.clone()]);
                return Some(self.n.clone());
            } else {
                let factors = &self.sieve[&self.n].clone();
                for factor in factors {
                    let v = factor.checked_add(&self.n)?;
                    if self.sieve.contains_key(&v) {
                        self.sieve.get_mut(&v).unwrap().push(factor.clone());
                    } else {
                        self.sieve.insert(v, vec![factor.clone()]);
                    }
                }
                self.sieve.remove(&self.n);
            }
        }
    }
}

crate::check_iteration_times!(
    Prime::new(), 22_000;
    PrimeGeneric::<u32>::new(), 46_000;
);

crate::check_sequences!(
    Prime::new(), 0, 10, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    Prime::new(), 1000, 10, [7927, 7933, 7937, 7949, 7951, 7963, 7993, 8009, 8011, 8017];
);
