use num::{BigInt, CheckedAdd, Integer};
use std::{
    collections::BTreeSet,
    fmt::{Debug, Display},
};

/// Starts with 1 and 2 and then every sum of sequential terms appears in ascending order.
///
/// 1, 2, 3, 5, 6, 8, 10, 11, 14, 16, 17, 18, 19, 21, 22, 24, 25, 29, 30, 32, 33, 34
pub struct A005243<T> {
    terms: Vec<T>,
    heap: BTreeSet<T>,
}

impl<T: Clone + Integer> A005243<T> {
    pub fn new() -> Self {
        Self {
            terms: vec![],
            heap: BTreeSet::new(),
        }
    }
}

impl A005243<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: CheckedAdd + Clone + Integer + Display + Debug> Iterator for A005243<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.terms.len() == 0 {
            self.heap.insert(T::one() + T::one());
            self.terms.push(T::one());
            return Some(T::one());
        }

        let out = self.heap.pop_first().unwrap();
        let mut s = out.clone();
        for value in self.terms.iter().rev() {
            s = s.checked_add(value)?;
            self.heap.insert(s.clone());
        }
        self.terms.push(out.clone());

        Some(out)
    }
}

crate::check_sequences!(
    A005243::<i32>::new(), [1, 2, 3, 5, 6, 8, 10, 11, 14, 16, 17, 18, 19, 21, 22, 24, 25, 29, 30, 32, 33, 34, 35, 37, 40, 41, 43, 45, 46, 47, 49, 51, 54, 57, 58, 59, 60, 62, 65, 67, 68, 69];
);
