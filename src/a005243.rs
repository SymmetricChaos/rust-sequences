use crate::transforms::complement::Complement;
use num::{BigInt, CheckedAdd, Integer};
use std::collections::BTreeSet;

/// Starts with 1 and 2 and then every sum of sequential terms appears in ascending order.
///
/// 1, 2, 3, 5, 6, 8, 10, 11, 14, 16, 17, 18, 19, 21, 22, 24, 25, 29, 30, 32, 33, 34
pub struct A005243<T> {
    terms: Vec<T>,
    heap: BTreeSet<T>,
}

impl<T: CheckedAdd + Clone + Integer> A005243<T> {
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

impl<T: CheckedAdd + Clone + Integer> Iterator for A005243<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.terms.len() == 0 {
            self.heap.insert(T::one() + T::one());
            self.terms.push(T::one());
            return Some(T::one());
        }

        // When a new value is determined there are only a few new sequences of sums created, all of which end with the new value
        // A BTreeSet is used as a priority queue to always extract the smallest value and ignore duplications
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

/// Complement of A005243, starting from 1.
///
/// 4, 7, 9, 12, 13, 15, 20, 23, 26, 27, 28...
pub struct A048973<T>(Complement<T>);

impl<T: CheckedAdd + Clone + Integer + 'static> A048973<T> {
    pub fn new() -> Self {
        A048973(Complement::new(A005243::new(), T::one()))
    }
}

impl A048973<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: CheckedAdd + Clone + Integer> Iterator for A048973<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

crate::check_sequences!(
    A005243::<i32>::new(), [1, 2, 3, 5, 6, 8, 10, 11, 14, 16, 17, 18, 19, 21, 22, 24, 25, 29, 30, 32, 33, 34, 35, 37, 40, 41, 43, 45, 46, 47, 49, 51, 54, 57, 58, 59, 60, 62, 65, 67, 68, 69, 70, 71, 72, 73, 75, 76, 77, 78, 80, 81, 82, 84, 86, 87, 88, 90, 91, 92, 93, 94, 95, 96, 97, 99, 100];
    A048973::<u32>::new(), [4, 7, 9, 12, 13, 15, 20, 23, 26, 27, 28, 31, 36, 38, 39, 42, 44, 48, 50, 52, 53, 55, 56, 61, 63, 64, 66, 74, 79, 83, 85, 89, 98, 101, 103, 107, 109, 114, 120, 123, 125, 128, 131, 133, 136, 144, 152, 157, 159, 160, 165, 168, 182, 184, 190, 192, 198, 203, 208, 212];
);
