use std::cmp::Ordering;

/// The terms that are greater than all previous terms in a seqeunce.
pub struct HighWaterMark<T> {
    iter: Box<dyn Iterator<Item = T>>,
    record: T,
    compare: Box<dyn Fn(&T, &T) -> Ordering>,
}

impl<T: Clone + Ord> HighWaterMark<T> {
    /// Create a high water mark sequence using the default ordering.
    pub fn new<I>(iter: I, start: T) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            iter: Box::new(iter),
            record: start,
            compare: Box::new(|a, b| a.cmp(b)),
        }
    }

    /// Create a high water mark sequence using a custom defined order. The ordering must be a total order.
    pub fn new_by<I, C>(iter: I, start: T, compare: C) -> Self
    where
        I: Iterator<Item = T> + 'static,
        C: Fn(&T, &T) -> Ordering + 'static,
    {
        Self {
            iter: Box::new(iter),
            record: start,
            compare: Box::new(compare),
        }
    }
}

impl<T: Clone + Ord> Iterator for HighWaterMark<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let val = self.iter.next()?;
            match (self.compare)(&val, &self.record) {
                Ordering::Greater => {
                    self.record = val.clone();
                    return Some(val);
                }
                _ => continue,
            }
        }
    }
}
