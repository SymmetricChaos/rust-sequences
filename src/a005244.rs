use num::{BigInt, CheckedMul, Integer};
use std::collections::BTreeSet;

/// Starting with 2 and 3 every number that can be written as (a*b)-1,  for a and b in the sequence, appears in increasing order.
///
/// 2, 3, 5, 9, 14, 17, 26, 27, 33, 41...
pub struct A005244<T> {
    terms: Vec<T>,
    products: BTreeSet<T>,
}

impl<T: Clone + Integer> A005244<T> {
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

crate::check_sequences!(
    A005244::<u32>::new(), [2, 3, 5, 9, 14, 17, 26, 27, 33, 41, 44, 50, 51, 53, 65, 69, 77, 80, 81, 84, 87, 98, 99, 101, 105, 122, 125, 129, 131, 134, 137, 149, 152, 153, 158, 159, 161, 164, 167, 173, 194, 195, 197, 201, 204, 206, 209, 219, 230, 233, 237, 239, 242, 243, 249];
);
