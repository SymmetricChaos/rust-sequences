use num::{CheckedAdd, One};

use crate::core::traits::Increment;

/// The complementary sequence is all the numbers that do not appear in it after some starting point, usually 0 or 1. This implementation only works for a strictly increasing sequence.
pub struct Complement<T> {
    iter: Box<dyn Iterator<Item = T>>,
    record: T,
    n: T,
}

impl<T: CheckedAdd + Clone + One + PartialEq> Complement<T> {
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
        }
    }
}

impl<T: CheckedAdd + Clone + One + PartialEq> Iterator for Complement<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.n == self.record {
                self.record = self.iter.next()?;
                self.n.incr()?;
            } else {
                let out = self.n.clone();
                self.n.incr()?;
                return Some(out);
            }
        }
    }
}

#[cfg(test)]
use crate::core::{parity::Evens, primes::Primes};
crate::check_sequences!(
    Complement::new(Evens::new(), 0), [1,3,5,7];
    Complement::new(Primes::new(), 0), [0,1,4,6,8,9,10,12,14,15,16];
);
