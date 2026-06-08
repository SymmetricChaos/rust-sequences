use crate::Number;
use num::{BigInt, CheckedAdd, Integer};
use std::{collections::HashSet, hash::Hash};

/// The forest fire sequence. Each term is the smallest value such that no three terms form an arithmetic sequences. The graph resembles wind-blown spoke.
///
/// ```text
/// 1, 1, 2, 1, 1, 2, 2, 4, 4, 1, 1, 2, 1, 1, 2, 2, 4, 4, 2, 4, 4, 5, 5...
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

#[cfg(feature = "big_int")]
impl ForestFire<BigInt> {
    pub fn new_big() -> Self {
        Self {
            terms: vec![],
            n: 0,
        }
    }
}

impl<T: Clone + CheckedAdd + Hash + Integer> Iterator for ForestFire<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.n;
        let mut i = 1;
        let mut j = T::one();
        let mut set = HashSet::new();
        while n >= 2 * i {
            let x = (self.terms[n - i]
                .clone()
                .checked_add(&self.terms[n - i].clone()))?
                - self.terms[n - 2 * i].clone();
            set.insert(x);
            i += 1;
            while set.contains(&j) {
                set.remove(&j);
                j = j + T::one();
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

crate::sample_sequences!(
    ForestFire::new();
);
