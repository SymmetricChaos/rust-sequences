use num::{CheckedAdd, CheckedSub, Integer, One, Zero, rational::Ratio};

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

/// Sequence of alternating partial sums. Returns None if overflow occurs or sequence ends.
pub struct PartialSumsAlternating<T> {
    sum: T,
    iter: Box<dyn Iterator<Item = T>>,
    subtract: bool,
}

impl<T: Zero> PartialSumsAlternating<T> {
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            sum: T::zero(),
            iter: Box::new(iter),
            subtract: false,
        }
    }
}

impl<T: CheckedAdd + CheckedSub + Clone> Iterator for PartialSumsAlternating<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.sum.clone();
        if self.subtract {
            self.sum = self.sum.checked_add(&self.iter.next()?)?;
        } else {
            self.sum = self.sum.checked_sub(&self.iter.next()?)?;
        }
        self.subtract = !self.subtract;
        Some(out)
    }
}

/// The partial sums of Cesaro's summation method.
pub struct CesaroPartialSums<T> {
    sum: T,
    iter: Box<dyn Iterator<Item = T>>,
    ctr: T,
}

impl<T: One + Zero> CesaroPartialSums<T> {
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

impl<T: Clone + CheckedAdd + Integer> Iterator for CesaroPartialSums<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr = self.ctr.checked_add(&T::one())?;
        self.sum = self.sum.checked_add(&self.iter.next()?)?;
        let out = Ratio::new(self.sum.clone(), self.ctr.clone());

        Some(out)
    }
}
