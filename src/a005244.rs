use crate::{Number, transforms::complement::Complement};
use num::{BigInt, CheckedAdd, CheckedMul, Integer, One};
use std::collections::BTreeSet;

/// Starting with 2 and 3 every number that can be written as (a*b)-1, for a and b in the sequence, appears in increasing order.
///
/// ```text
/// 2, 3, 5, 9, 14, 17, 26, 27, 33, 41, 44, 50, 51, 53, 65, 69, 77, 80...
/// ```
pub struct A005244<T> {
    terms: Vec<T>,
    products: BTreeSet<T>,
}

impl A005244<Number> {
    pub fn new() -> Self {
        Self {
            terms: vec![2],
            products: BTreeSet::from([3]),
        }
    }
}

impl A005244<BigInt> {
    pub fn new_big() -> Self {
        Self {
            terms: vec![BigInt::from(2)],
            products: BTreeSet::from([BigInt::from(3)]),
        }
    }
}

impl<T: Clone + CheckedMul + Integer> Iterator for A005244<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.terms.last().cloned();
        let n = self.products.pop_first().unwrap();
        for t in self.terms.iter() {
            self.products.insert(n.checked_mul(t)? - T::one());
        }
        self.terms.push(n);
        out
    }
}

/// Complement of A005244.
///
/// ```text
/// 1, 4, 6, 7, 8, 10, 11, 12, 13, 15, 16, 18, 19, 20, 21, 22, 23, 24...
/// ```
pub struct A171413<T>(Complement<T>);

impl A171413<Number> {
    pub fn new() -> Self {
        A171413(Complement::new(A005244::new(), 1))
    }
}

impl A171413<BigInt> {
    pub fn new_big() -> Self {
        A171413(Complement::new(A005244::new_big(), BigInt::one()))
    }
}

impl<T: Clone + CheckedAdd + CheckedMul + Integer> Iterator for A171413<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

crate::check_sequences!(
    A005244::new(), [2, 3, 5, 9, 14, 17, 26, 27, 33, 41, 44, 50, 51, 53, 65, 69, 77, 80, 81, 84, 87, 98, 99, 101, 105, 122, 125, 129, 131, 134, 137, 149, 152, 153, 158, 159, 161, 164, 167, 173, 194, 195, 197, 201, 204, 206, 209, 219, 230, 233, 237, 239, 242, 243, 249];
    A171413::new(), [1, 4, 6, 7, 8, 10, 11, 12, 13, 15, 16, 18, 19, 20, 21, 22, 23, 24, 25, 28, 29, 30, 31, 32, 34, 35, 36, 37, 38, 39, 40, 42, 43, 45, 46, 47, 48, 49, 52, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 66, 67, 68, 70, 71, 72, 73, 74, 75, 76, 78, 79, 82, 83, 85, 86, 88];
);

crate::sample_sequences!(
    A005244::new();
    A171413::new();
);
