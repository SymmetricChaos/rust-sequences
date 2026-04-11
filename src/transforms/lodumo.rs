use std::collections::BTreeSet;

use num::{CheckedAdd, Integer};

pub struct Lodumo<T> {
    iter: Box<dyn Iterator<Item = T>>,
    m: T,
    terms: BTreeSet<T>,
}

impl<T: CheckedAdd + Clone + Integer> Lodumo<T> {
    pub fn new<I>(iter: I, m: T) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            iter: Box::new(iter),
            m,
            terms: BTreeSet::new(),
        }
    }
}

impl<T: CheckedAdd + Clone + Integer> Iterator for Lodumo<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.iter.next()?;
        let mut reduced = n % self.m.clone();

        while self.terms.contains(&reduced) {
            reduced = reduced.checked_add(&self.m)?;
        }

        self.terms.insert(reduced.clone());

        Some(reduced)
    }
}
