use crate::core::traits::Increment;
use num::{CheckedAdd, One, Zero};

/// For a strictly increasing sequence the number of terms less than or equal to each natural number, typically starting with either 0 or 1.
pub struct Counting<T> {
    iter: Box<dyn Iterator<Item = T>>,
    record: T,
    n: T,
    count: T,
}

impl<T: CheckedAdd + Clone + One + PartialOrd + Zero> Counting<T> {
    pub fn new<I>(iter: I, start: T) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        let mut iter = Box::new(iter);
        let record = iter.next().unwrap();
        Self {
            iter,
            record,
            n: start,
            count: T::zero(),
        }
    }
}

impl<T: CheckedAdd + Clone + One + PartialOrd + Zero> Iterator for Counting<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.n.partial_cmp(&self.record) {
            Some(o) => match o {
                // If we haven't reached the record keep going without changing the count
                std::cmp::Ordering::Less => {
                    self.n.incr()?;
                }
                // If we reach the record increment the count, find the next value, and keep going
                std::cmp::Ordering::Equal => {
                    self.count.incr()?;
                    self.record = self.iter.next()?;
                    self.n.incr()?;
                }
                // If we have exceeded the record there's been a problem and we'll stop iteration
                std::cmp::Ordering::Greater => return None,
            },
            None => return None,
        }
        return Some(self.count.clone());
    }
}

#[cfg(test)]
use crate::core::{parity::Evens, primes::Primes};
crate::check_sequences!(
    Counting::new(Evens::new(), 0), [1, 1, 2, 2, 3, 3, 4, 4, 5];
    Counting::new(Primes::new(), 1), [0, 1, 2, 2, 3, 3, 4, 4, 4, 4, 5, 5, 6, 6, 6, 6, 7, 7, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 10, 10, 11, 11, 11, 11, 11, 11, 12, 12, 12, 12, 13, 13, 14, 14, 14, 14, 15, 15, 15, 15, 15, 15, 16, 16, 16, 16, 16, 16, 17, 17, 18, 18, 18, 18, 18, 18, 19, 19, 19, 19, 20, 20, 21, 21, 21, 21, 21, 21];
);
