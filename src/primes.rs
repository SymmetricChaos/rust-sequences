use num::BigInt;
use std::collections::HashMap; // Found to be much faster than BTreeMap

/// The prime numbers.
/// 2, 3, 5, 7, 11, 13, 17, 19, 23, 29...
pub struct Prime {
    sieve: HashMap<BigInt, Vec<BigInt>>,
    n: BigInt,
}

impl Prime {
    pub fn new() -> Prime {
        Prime {
            sieve: HashMap::<BigInt, Vec<BigInt>>::new(),
            n: BigInt::from(1),
        }
    }
}

impl Iterator for Prime {
    type Item = BigInt;

    fn next(&mut self) -> Option<BigInt> {
        loop {
            self.n += 1;
            if !self.sieve.contains_key(&self.n) {
                self.sieve.insert(&self.n + &self.n, vec![self.n.clone()]);
                return Some(self.n.clone());
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
            }
        }
    }
}

crate::check_time!(
    Prime::new(), 100_000;
);

crate::check_sequences!(
    Prime::new(), 0, 10, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    Prime::new(), 1000, 10, [7927, 7933, 7937, 7949, 7951, 7963, 7993, 8009, 8011, 8017];
);
