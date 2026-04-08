use std::ops::Neg;

use num::{CheckedMul, One, Signed};

/// Sequence of partial products. Returns None if overflow occurs or sequence ends.
pub struct PartialProds<T> {
    prod: T,
    iter: Box<dyn Iterator<Item = T>>,
}

impl<T: CheckedMul + Clone + One> PartialProds<T> {
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

/// Sequence of alternating partial products. Returns None if overflow occurs or sequence ends.
pub struct AlternatingPartialProds<T> {
    prod: T,
    iter: Box<dyn Iterator<Item = T>>,
    positive: bool,
}

impl<T: CheckedMul + Clone + Neg + One> AlternatingPartialProds<T> {
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            prod: T::one(),
            iter: Box::new(iter),
            positive: true,
        }
    }
}

impl<T: CheckedMul + Clone + Signed> Iterator for AlternatingPartialProds<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.prod.clone();
        if self.positive {
            self.prod = self.prod.checked_mul(&self.iter.next()?)?;
        } else {
            self.prod = self.prod.checked_mul(&-self.iter.next()?)?;
        }
        self.positive = !self.positive;
        Some(out)
    }
}
