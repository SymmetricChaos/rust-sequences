use num::{BigInt, CheckedAdd, One};
use std::collections::VecDeque;

use crate::Number;

/// A generalization of the Fibonacci and Narayana sequences. For a non-negative constant, k, the sequence begins with the number 1 repeated k+1) times. Further terms are equal to f(n) = f(n-1) + f(n-k). Any of these sequences may be prefixed by a single zero.
///
/// ```text
/// k = 0 (powers of two)
/// 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192...
///
/// k = 1 (Fibonacci sequence)
/// 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597...
///
/// k = 2 (Narayana cow sequence)
/// 1, 1, 1, 2, 3, 4, 6, 9, 13, 19, 28, 41, 60, 88, 129, 189, 277, 406...
///
/// k = 3 (A003269)
/// 1, 1, 1, 1, 2, 3, 4, 5, 7, 10, 14, 19, 26, 36, 50, 69, 95, 131, 181...
/// ```
pub struct NarayanaGeneral<T> {
    window: VecDeque<T>,
}

impl NarayanaGeneral<Number> {
    pub fn new(k: usize) -> Self {
        Self {
            window: VecDeque::from(vec![1; k + 1]),
        }
    }
}

#[cfg(feature = "big_int")]
impl NarayanaGeneral<BigInt> {
    pub fn new_big(k: usize) -> Self {
        Self {
            window: VecDeque::from(vec![BigInt::one(); k + 1]),
        }
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
    NarayanaGeneral::new(0), [1, 2, 4, 8, 16, 32, 64, 128]; // A000079
    NarayanaGeneral::new(1), [1, 1, 2, 3, 5, 8, 13, 21, 34]; // A000045
    NarayanaGeneral::new(2), [1, 1, 1, 2, 3, 4, 6, 9, 13, 19]; // A000930
    NarayanaGeneral::new(3), [1, 1, 1, 1, 2, 3, 4, 5, 7, 10]; // A003269
);

crate::sample_sequences!(
    NarayanaGeneral::new(0);
    NarayanaGeneral::new(1);
    NarayanaGeneral::new(2);
    NarayanaGeneral::new(3);
);
