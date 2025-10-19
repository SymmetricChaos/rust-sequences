use num::{CheckedMul, One};

/// Sequence of partial products. Returns None if overflow occurs or sequence ends.
pub struct PartialProds<T> {
    prod: T,
    iter: Box<dyn Iterator<Item = T>>,
}

impl<T: One> PartialProds<T> {
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            prod: T::one(),
            iter: Box::new(iter),
        }
    }
}

impl<T: CheckedMul + Clone> Iterator for PartialProds<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.prod.clone();
        self.prod = self.prod.checked_mul(&self.iter.next()?)?;
        Some(out)
    }
}
