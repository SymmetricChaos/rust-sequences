use num::{BigInt, One};

use crate::Number;
use std::collections::HashSet;

/// The forest fire sequence. Each term is the smallest value such that no three terms form an arithmetic sequences. The graph resembles wind-blown spoke.
///
/// ```text
/// 1, 1, 2, 1, 1, 2, 2, 4, 4, 1, 1, 2, 1, 1, 2, 2...
/// ```
pub struct ForestFire<T> {
    terms: Vec<T>,
    n: usize,
}

impl ForestFire<Number> {
    pub fn new() -> Self {
        Self {
            terms: vec![],
            n: 0,
        }
    }
}

impl ForestFire<BigInt> {
    pub fn new_big() -> Self {
        Self {
            terms: vec![],
            n: 0,
        }
    }
}

impl Iterator for ForestFire<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.n;
        let mut i = 1;
        let mut j = 1;
        let mut set = HashSet::new();
        while n >= 2 * i {
            match (2 * self.terms[n - i]).checked_sub(self.terms[n - 2 * i]) {
                Some(x) => {
                    set.insert(x);
                }
                None => (),
            }
            i += 1;
            while set.contains(&j) {
                set.remove(&j);
                j += 1;
            }
        }
        self.terms.push(j);

        let out = self.terms[n];
        self.n = self.n.checked_add(1)?;

        Some(out)
    }
}

impl Iterator for ForestFire<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.n;
        let mut i = 1;
        let mut j = BigInt::one();
        let mut set = HashSet::new();
        while n >= 2 * i {
            let x = (self.terms[n - i].clone() * 2) - self.terms[n - 2 * i].clone();
            set.insert(x);
            i += 1;
            while set.contains(&j) {
                set.remove(&j);
                j += 1;
            }
        }
        self.terms.push(j);

        let out = self.terms[n].clone();
        self.n = self.n.checked_add(1)?;

        Some(out)
    }
}

crate::check_iteration_times!(
    ForestFire::new(), [100, 1000, 10_000, 100_000];
);

crate::check_sequences!(
    ForestFire::new(), [1, 1, 2, 1, 1, 2, 2, 4, 4, 1, 1, 2, 1, 1, 2, 2, 4, 4, 2, 4, 4, 5, 5, 8, 5, 5, 9, 1, 1, 2, 1, 1, 2, 2, 4, 4, 1, 1, 2, 1, 1, 2, 2, 4, 4, 2, 4, 4, 5, 5, 8, 5, 5, 9, 9, 4, 4, 5, 5, 10, 5, 5, 10, 2, 10, 13, 11, 10, 8, 11, 13, 10, 12, 10, 10, 12, 10, 11, 14, 20, 13];
);
