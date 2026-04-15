use num::{CheckedAdd, Integer};
use std::collections::BTreeMap;

/// The Lodumo_m transform of an iterator. Each term is the smallest natural number not previously which is a multple of that term of the original iterator modulo m.
pub struct Lodumo<T> {
    iter: Box<dyn Iterator<Item = T>>,
    m: T,
    terms: BTreeMap<T, T>,
}

impl<T: CheckedAdd + Clone + Integer> Lodumo<T> {
    pub fn new<I>(iter: I, m: T) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            iter: Box::new(iter),
            m,
            terms: BTreeMap::new(),
        }
    }
}

impl<T: CheckedAdd + Clone + Integer> Iterator for Lodumo<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.iter.next()?;
        let reduced = n % self.m.clone();

        match self.terms.get_mut(&reduced) {
            Some(x) => {
                *x = x.checked_add(&self.m)?;
            }
            None => {
                self.terms.insert(reduced.clone(), reduced.clone());
            }
        }

        self.terms.get(&reduced).cloned()
    }
}

// A160081: Lodumo_5 of the Fibonacci numbers.
crate::check_sequences!(
    Lodumo::new(
        [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229].into_iter(), 5
        ),
    [0, 1, 6, 2, 3, 5, 8, 13, 11, 4, 10, 9, 14, 18, 7, 15, 12, 17, 19, 16, 20, 21, 26, 22, 23, 25, 28, 33, 31, 24];
);
