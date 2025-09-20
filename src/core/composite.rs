use num::{BigInt, CheckedAdd, One, PrimInt, Zero};
use std::{collections::HashMap, hash::Hash}; // Found to be much faster than BTreeMap

/// The composite numbers. A002808
/// 4, 6, 8, 9, 10, 12, 14, 15, 16...
pub struct Composite<T> {
    sieve: HashMap<T, Vec<T>>,
    n: T,
}

impl<T: PrimInt> Composite<T> {
    pub fn new() -> Self {
        Self {
            sieve: HashMap::<T, Vec<T>>::new(),
            n: T::one(),
        }
    }
}

impl Composite<BigInt> {
    pub fn new_big() -> Self {
        Self {
            sieve: HashMap::<BigInt, Vec<BigInt>>::new(),
            n: BigInt::from(1),
        }
    }
}

impl<T: Zero + One + CheckedAdd + Clone + Hash + Eq> Iterator for Composite<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.n = self.n.checked_add(&T::one())?;
            if !self.sieve.contains_key(&self.n) {
                let v = self.n.checked_add(&self.n)?;
                self.sieve.insert(v, vec![self.n.clone()]);
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
                return Some(self.n.clone());
            }
        }
    }
}

crate::check_sequences!(
    Composite::new_big(), 0, 10, [4, 6, 8, 9, 10, 12, 14, 15, 16, 18];
);
