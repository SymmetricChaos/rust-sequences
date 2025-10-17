use num::{CheckedSub, Signed};

/// The absolute differences of every adjecent pair from a sequence. |f(x+1) - f(x)|
pub struct AbsDiffs<T> {
    prev: T,
    iter: Box<dyn Iterator<Item = T>>,
}

impl<T> AbsDiffs<T> {
    pub fn new<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            prev: iter.next().unwrap(),
            iter: Box::new(iter),
        }
    }
}

impl<T: CheckedSub + Clone + Ord> Iterator for AbsDiffs<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.iter.next()?;
        let out = if self.prev > cur {
            self.prev.checked_sub(&cur)?
        } else {
            cur.checked_sub(&self.prev)?
        };
        self.prev = cur;
        Some(out)
    }
}

/// The first differences of a sequence: f(x+1) - f(x)
/// Requires signed values
pub struct Diffs<T> {
    prev: T,
    iter: Box<dyn Iterator<Item = T>>,
}

impl<T: Signed> Diffs<T> {
    pub fn new<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            prev: iter.next().unwrap(),
            iter: Box::new(iter),
        }
    }
}

impl<T: CheckedSub + Clone + Signed> Iterator for Diffs<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.iter.next()?;
        let out = cur.clone().checked_sub(&self.prev.clone())?;
        self.prev = cur;
        Some(out)
    }
}
