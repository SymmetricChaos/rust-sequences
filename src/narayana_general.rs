use std::collections::VecDeque;

use num::{BigInt, CheckedAdd, One};

/// A generalization of the Fibonacci and Narayana sequences. For a non-negative constant, k, the sequence begins with the number 1 repeated k+1) times. Further terms are equal to f(n-k) + f(n).
/// k = 0 gives the powers of two
/// k = 1 gives the Fibonacci sequence
/// k = 2 gives the Narayana cow sequence
pub struct NarayanaGeneral<T> {
    window: VecDeque<T>,
}

impl<T: One + Clone> NarayanaGeneral<T> {
    pub fn new(k: usize) -> Self {
        Self {
            window: VecDeque::from(vec![T::one(); k + 1]),
        }
    }
}

impl NarayanaGeneral<BigInt> {
    pub fn new_big(k: usize) -> Self {
        Self::new(k)
    }
}

impl<T: Clone + CheckedAdd> Iterator for NarayanaGeneral<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let tail = self.window.iter().last()?.clone();
        let head = self.window.pop_front()?;
        self.window.push_back(head.checked_add(&tail)?);

        Some(head)
    }
}
crate::check_sequences!(
    NarayanaGeneral::<i32>::new(0), [1, 2, 4, 8, 16, 32, 64, 128];
    NarayanaGeneral::<i32>::new(1), [1, 1, 2, 3, 5, 8, 13, 21, 34];
    NarayanaGeneral::<i32>::new(2), [1, 1, 1, 2, 3, 4, 6, 9, 13, 19];
);
