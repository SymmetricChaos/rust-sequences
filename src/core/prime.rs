use num::{BigInt, CheckedAdd, One, PrimInt, Zero};
use std::{collections::HashMap, hash::Hash}; // Found to be much faster than BTreeMap

/// The prime natural numbers.
/// 2, 3, 5, 7, 11, 13, 17, 19, 23, 29...
pub struct Prime<T> {
    sieve: HashMap<T, Vec<T>>,
    n: T,
}

impl<T: PrimInt> Prime<T> {
    pub fn new_prim() -> Self {
        Self {
            sieve: HashMap::<T, Vec<T>>::new(),
            n: T::one(),
        }
    }

    /// An older definition of primes also known as the non-composite numbers.
    /// 1, 2, 3, 5, 7, 11, 13, 17, 19, 23...
    pub fn with_one_prim() -> Self {
        Self {
            sieve: HashMap::<T, Vec<T>>::new(),
            n: T::zero(),
        }
    }
}

impl Prime<BigInt> {
    pub fn new() -> Self {
        Self {
            sieve: HashMap::<BigInt, Vec<BigInt>>::new(),
            n: BigInt::one(),
        }
    }

    /// An older definition of primes also known as the non-composite numbers.
    /// 1, 2, 3, 5, 7, 11, 13, 17, 19, 23...
    pub fn with_one() -> Self {
        Self {
            sieve: HashMap::<BigInt, Vec<BigInt>>::new(),
            n: BigInt::zero(),
        }
    }
}

impl<T: Zero + One + CheckedAdd + Clone + Hash + Eq> Iterator for Prime<T> {
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
    Prime::new(), 21_000;
    Prime::<u32>::new_prim(), 21_000;
);

crate::check_sequences!(
    Prime::new(), 0, 10, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    Prime::new(), 1000, 10, [7927, 7933, 7937, 7949, 7951, 7963, 7993, 8009, 8011, 8017];
);
