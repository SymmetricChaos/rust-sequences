use num::{CheckedAdd, CheckedSub, Integer, Zero, rational::Ratio};

/// Sequence of partial sums. Returns None if overflow occurs during addition or if the sequence ends.
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

/// Sequence of alternating partial sums. Returns None if overflow occurs during addition or subtraction or if the sequence ends.
pub struct AlternatingPartialSums<T> {
    sum: T,
    iter: Box<dyn Iterator<Item = T>>,
    positive: bool,
}

impl<T: CheckedAdd + CheckedSub + Clone + Zero> AlternatingPartialSums<T> {
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            sum: T::zero(),
            iter: Box::new(iter),
            positive: true,
        }
    }
}

impl<T: CheckedAdd + CheckedSub + Clone> Iterator for AlternatingPartialSums<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.sum.clone();
        if self.positive {
            self.sum = self.sum.checked_add(&self.iter.next()?)?;
        } else {
            self.sum = self.sum.checked_sub(&self.iter.next()?)?;
        }
        self.positive = !self.positive;
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
