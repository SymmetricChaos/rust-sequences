use num::{CheckedAdd, CheckedMul, One, Zero, rational::Ratio};

/// Sequence of partial sums. Returns None if overflow occurs or sequence ends.
pub struct PartialSums<T> {
    sum: T,
    iter: Box<dyn Iterator<Item = T>>,
}

impl<T: Zero> PartialSums<T> {
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            sum: T::zero(),
            iter: Box::new(iter),
        }
    }
}

impl<T: CheckedAdd + Clone> Iterator for PartialSums<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.sum.clone();
        self.sum = self.sum.checked_add(&self.iter.next()?)?;
        Some(out)
    }
}

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

/// Sequence of numerators of a sequence of ratios.
pub struct Numerators<T> {
    iter: Box<dyn Iterator<Item = Ratio<T>>>,
}

impl<T: One> Numerators<T> {
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = Ratio<T>> + 'static,
    {
        Self {
            iter: Box::new(iter),
        }
    }
}

impl<T: Clone> Iterator for Numerators<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.iter.next()?.numer().clone())
    }
}

/// Sequence of denominators of a sequence of ratios.
pub struct Denominator<T> {
    iter: Box<dyn Iterator<Item = Ratio<T>>>,
}

impl<T: One> Denominator<T> {
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = Ratio<T>> + 'static,
    {
        Self {
            iter: Box::new(iter),
        }
    }
}

impl<T: Clone> Iterator for Denominator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.iter.next()?.denom().clone())
    }
}
