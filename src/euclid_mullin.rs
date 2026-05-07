use num::CheckedMul;

use crate::utils::divisibility::prime_factorization;

/// The Euclid-Mullin sequence defined using the smallest prime factors of the products.
///
/// 2, 3, 7, 43, 13, 53, 5, 6221671...
pub struct EuclidMullinSmallest {
    terms: Vec<u64>,
    overflowed: bool,
}

impl EuclidMullinSmallest {
    pub fn new() -> Self {
        Self {
            terms: vec![2],
            overflowed: false,
        }
    }
}

impl Iterator for EuclidMullinSmallest {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.overflowed {
            return None;
        }
        let out = self.terms.last().cloned();

        let mut new = 1;
        for t in self.terms.iter() {
            new = match new.checked_mul(t) {
                Some(n) => n,
                None => {
                    self.overflowed = true;
                    return out;
                }
            }
        }
        new += 1;
        self.terms.push(prime_factorization(new)[0].0);

        out
    }
}

/// The Euclid-Mullin sequence defined using the largest prime factors of the products.
///
/// 2, 3, 7, 43, 13, 53, 5, 6221671...
pub struct EuclidMullinLargest {
    terms: Vec<u64>,
    overflowed: bool,
}

impl EuclidMullinLargest {
    pub fn new() -> Self {
        Self {
            terms: vec![2],
            overflowed: false,
        }
    }
}

impl Iterator for EuclidMullinLargest {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.overflowed {
            return None;
        }
        let out = self.terms.last().cloned();

        let mut new = 1;
        for t in self.terms.iter() {
            new = match new.checked_mul(t) {
                Some(n) => n,
                None => {
                    self.overflowed = true;
                    return out;
                }
            }
        }
        new += 1;
        self.terms.push(prime_factorization(new).last().unwrap().0);

        out
    }
}

crate::check_sequences!(
    EuclidMullinSmallest::new(), [2_u64, 3, 7, 43, 13, 53, 5, 6221671, 38709183810571]; // limited by overflow
    EuclidMullinLargest::new(), [2_u64, 3, 7, 43, 139, 50207, 340999, 2365347734339]; // limited by overflow
);
