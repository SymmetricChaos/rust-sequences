use crate::core::traits::Increment;
use num::{CheckedAdd, Integer};

/// The run length encoding of an integer sequence. A sequence of pairs of numbers where the first is an integer from the original sequence and the second is the number of times it is repeated.
pub struct RunLengthEncoding<T> {
    iter: Box<dyn Iterator<Item = T>>,
    val: Option<T>,
}

impl<T: CheckedAdd + Clone + Integer> RunLengthEncoding<T> {
    /// Returns tuples of the form (T,T).
    pub fn new<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        let val = iter.next();
        Self {
            val: val,
            iter: Box::new(iter),
        }
    }

    /// A flattened version of the run length encoding.
    pub fn new_flat<I>(iter: I) -> impl Iterator<Item = T>
    where
        I: Iterator<Item = T> + 'static,
    {
        Self::new(iter).map(|(a, b)| [a, b]).flatten()
    }
}

impl<T: CheckedAdd + Clone + Integer> Iterator for RunLengthEncoding<T> {
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.val.clone();
        let mut ctr = T::one();
        loop {
            let n = self.iter.next();
            if n == val {
                ctr.incr()?;
            } else {
                self.val = n;
                break;
            }
        }

        Some((val.unwrap(), ctr))
    }
}

crate::print_sequences!(
    RunLengthEncoding::new([1, 1, 3, 3, 3, 3, 5, 5, 5, 5, 5, 5, 7, 7, 7, 7, 7, 7, 7, 7].into_iter()), 4, "{:?}", ", ";
    RunLengthEncoding::new_flat([1, 1, 3, 3, 3, 3, 5, 5, 5, 5, 5, 5, 7, 7, 7, 7, 7, 7, 7, 7].into_iter()), 8;
);
