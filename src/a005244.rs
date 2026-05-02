use crate::transforms::complement::Complement;
use num::{BigInt, CheckedAdd, CheckedMul, Integer};
use std::collections::BTreeSet;

/// Starting with 2 and 3 every number that can be written as (a*b)-1,  for a and b in the sequence, appears in increasing order.
///
/// 2, 3, 5, 9, 14, 17, 26, 27, 33, 41...
pub struct A005244<T> {
    terms: Vec<T>,
    products: BTreeSet<T>,
}

impl<T: Clone + Integer + CheckedMul> A005244<T> {
    pub fn new() -> Self {
        Self {
            terms: vec![T::one() + T::one()],
            products: BTreeSet::from([T::one() + T::one() + T::one()]),
        }
    }
}

impl A005244<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Clone + Integer + CheckedMul> Iterator for A005244<T> {
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

/// Complement of A005244, starting from 1.
///
/// 1, 4, 6, 7, 8, 10, 11, 12, 13, 15, 16...
pub struct A171413<T>(Complement<T>);

impl<T: Clone + Integer + CheckedMul + CheckedAdd + 'static> A171413<T> {
    pub fn new() -> Self {
        A171413(Complement::new(A005244::new(), T::one()))
    }
}

impl A171413<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Clone + Integer + CheckedMul + CheckedAdd> Iterator for A171413<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

crate::check_sequences!(
    A005244::<u32>::new(), [2, 3, 5, 9, 14, 17, 26, 27, 33, 41, 44, 50, 51, 53, 65, 69, 77, 80, 81, 84, 87, 98, 99, 101, 105, 122, 125, 129, 131, 134, 137, 149, 152, 153, 158, 159, 161, 164, 167, 173, 194, 195, 197, 201, 204, 206, 209, 219, 230, 233, 237, 239, 242, 243, 249];
    A171413::<u32>::new(), [1, 4, 6, 7, 8, 10, 11, 12, 13, 15, 16, 18, 19, 20, 21, 22, 23, 24, 25, 28, 29, 30, 31, 32, 34, 35, 36, 37, 38, 39, 40, 42, 43, 45, 46, 47, 48, 49, 52, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 66, 67, 68, 70, 71, 72, 73, 74, 75, 76, 78, 79, 82, 83, 85, 86, 88];
);
