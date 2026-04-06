use num::{CheckedAdd, Integer, Zero, rational::Ratio};

/// Sequence of partial sums. Returns None if overflow occurs or sequence ends.
pub struct PartialSums<T> {
    sum: T,
    iter: Box<dyn Iterator<Item = T>>,
}

impl<T: CheckedAdd + Clone + Zero> PartialSums<T> {
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

/// The partial sums of Cesaro's summation method.
pub struct CesaroPartialSums<T> {
    sum: T,
    iter: Box<dyn Iterator<Item = T>>,
    ctr: T,
}

impl<T: CheckedAdd + Clone + Integer> CesaroPartialSums<T> {
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            sum: T::zero(),
            iter: Box::new(iter),
            ctr: T::zero(),
        }
    }
}

impl<T: CheckedAdd + Clone + Integer> Iterator for CesaroPartialSums<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr = self.ctr.checked_add(&T::one())?;
        self.sum = self.sum.checked_add(&self.iter.next()?)?;
        let out = Ratio::new(self.sum.clone(), self.ctr.clone());

        Some(out)
    }
}
