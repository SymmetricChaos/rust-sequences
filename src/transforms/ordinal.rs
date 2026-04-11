use std::{collections::HashMap, hash::Hash};

/// The ordinal transform of a sequence. The number of times the entry at n has occured since the start of the sequence.
pub struct OrdinalTransform<T> {
    iter: Box<dyn Iterator<Item = T>>,
    map: HashMap<T, usize>,
}

impl<T: Clone + Eq + Hash> OrdinalTransform<T> {
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            iter: Box::new(iter),
            map: HashMap::new(),
        }
    }
}

impl<T: Clone + Eq + Hash> Iterator for OrdinalTransform<T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let term = self.iter.next()?;
        self.map
            .entry(term.clone())
            .and_modify(|c| *c += 1)
            .or_insert(1);
        self.map.get(&term).copied()
    }
}
