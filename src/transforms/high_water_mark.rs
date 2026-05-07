use std::cmp::Ordering;

/// The terms of a seqeunce that are greater than all previous terms in that seqeunce.
pub struct HighWaterMark<T> {
    iter: Box<dyn Iterator<Item = T>>,
    record: T,
    compare: Box<dyn Fn(&T, &T) -> Ordering>,
}

impl<T: Clone + Ord> HighWaterMark<T> {
    /// Create a high water mark sequence using the default ordering.
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        let mut iter = Box::new(iter);
        let record = iter.next().unwrap();
        Self {
            iter,
            record,
            compare: Box::new(|a, b| a.cmp(b)),
        }
    }
}

impl<T: Clone> HighWaterMark<T> {
    /// Create a high water mark sequence using a custom defined order. The ordering must be a total order.
    pub fn new_by<I, C>(iter: I, compare: C) -> Self
    where
        I: Iterator<Item = T> + 'static,
        C: Fn(&T, &T) -> Ordering + 'static,
    {
        let mut iter = Box::new(iter);
        let record = iter.next().unwrap();
        Self {
            iter,
            record,
            compare: Box::new(compare),
        }
    }
}

impl<T: Clone> Iterator for HighWaterMark<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let val = self.iter.next()?;
            match (self.compare)(&val, &self.record) {
                Ordering::Greater => {
                    let out = self.record.clone(); // this allows always returning the first term
                    self.record = val.clone();
                    return Some(out);
                }
                _ => continue,
            }
        }
    }
}

crate::check_sequences!(
    HighWaterMark::<i32>::new(crate::core::EvenIntegers::new()), [0, 2, 4, 6, 8];
    HighWaterMark::<i32>::new_by(crate::core::EvenIntegers::new(), |a,b| b.cmp(a)), [0, -2, -4, -6];
);
