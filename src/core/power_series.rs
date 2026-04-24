use num::{CheckedAdd, CheckedMul, One, Zero};

/// Partial sums of a power series evaluated at some point.
pub struct PowerSeries<T> {
    iter: Box<dyn Iterator<Item = T>>,
    x: T,
    sum: T,
    prod: T,
}

impl<T: CheckedAdd + CheckedMul + Clone + One + Zero> PowerSeries<T> {
    pub fn new<I>(iter: I, x: T) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            iter: Box::new(iter),
            x,
            sum: T::zero(),
            prod: T::one(),
        }
    }
}

impl<T: CheckedAdd + CheckedMul + Clone> Iterator for PowerSeries<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.sum = self
            .iter
            .next()?
            .checked_mul(&self.prod)?
            .checked_add(&self.sum)?;

        Some(self.sum.clone())
    }
}
