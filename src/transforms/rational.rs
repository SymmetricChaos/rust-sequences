use num::{Integer, rational::Ratio};

/// Sequence of reciprocals of a sequence of integers. Returns None if the integer is zero.
pub struct IntegerReciprocals<T> {
    iter: Box<dyn Iterator<Item = T>>,
}

impl<T: Clone + Integer> IntegerReciprocals<T> {
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            iter: Box::new(iter),
        }
    }
}

impl<T: Clone + Integer> Iterator for IntegerReciprocals<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.iter.next()?;
        if n.is_zero() {
            None
        } else {
            Some(Ratio::<T>::new(T::one(), n))
        }
    }
}

/// Given integer sequences N and D return the sequence n_i/d_i for each element of N and D.
/// Returns None if d_i is zero.
pub struct Ratios<T> {
    n: Box<dyn Iterator<Item = T>>,
    d: Box<dyn Iterator<Item = T>>,
}

impl<T> Ratios<T> {
    pub fn new<N, D>(n: N, d: D) -> Self
    where
        N: Iterator<Item = T> + 'static,
        D: Iterator<Item = T> + 'static,
    {
        Self {
            n: Box::new(n),
            d: Box::new(d),
        }
    }
}

impl<T: Clone + Integer> Iterator for Ratios<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // Often iteration will stop if either num or den is None but if it doesn't we must ensure they do not get out of sync.
        let num = self.n.next();
        let den = self.d.next();

        if let (Some(n), Some(d)) = (num, den) {
            if d.is_zero() {
                None
            } else {
                Some(Ratio::<T>::new(n, d))
            }
        } else {
            None
        }
    }
}
