use num::{BigInt, Integer, Zero};

use crate::Number;

/// The Baum-Sweet sequence. Characteristic function of non-negative intgers which have binary expansions that never contain an odd number of sequential 0s.
///
/// ```text
/// 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0...
/// ```
pub struct BaumSweet<T> {
    terms: Vec<T>, // TODO: how to trim this to reduce storage?
    ctr: usize,
}

impl BaumSweet<Number> {
    pub fn new() -> Self {
        Self {
            terms: vec![0],
            ctr: 0,
        }
    }
}

impl BaumSweet<BigInt> {
    pub fn new_big() -> Self {
        Self {
            terms: vec![BigInt::zero()],
            ctr: 0,
        }
    }
}

impl<T: Clone + Integer> Iterator for BaumSweet<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = if self.ctr.is_zero() {
            T::zero()
        } else {
            T::one() - self.terms[self.ctr].clone()
        };

        self.ctr += 1;
        let mut n = self.ctr.clone();

        while (n % 4).is_zero() {
            n = n / 4;
        }
        if n.is_even() {
            self.terms.push(T::one());
        } else {
            self.terms.push(self.terms[(n - 1) / 2].clone());
        }

        Some(out)
    }
}

crate::check_sequences!(
    BaumSweet::new_big(), [0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0];
);

crate::sample_sequences!(
    BaumSweet::new();
);
