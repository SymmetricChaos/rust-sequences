use std::marker::PhantomData;

use num::{BigInt, CheckedAdd, CheckedDiv, Integer};

use crate::odd_part::OddPart;

/// The regular paperfolding sequence, derived from the direction of folds that ooccur when a sheet of paper is folded in half in the same direction repeatedly. Each term is 1 if the odd part of n is equal to 1 modulo 4 and otherwise is 0.
///
/// 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0...
pub struct RegularPaperfolding<T> {
    odd_part: OddPart<usize>,
    _phantom: PhantomData<T>,
}

impl<T: Clone + Integer + CheckedDiv + CheckedAdd> RegularPaperfolding<T> {
    pub fn new() -> Self {
        Self {
            odd_part: OddPart::new(),
            _phantom: PhantomData,
        }
    }
}

impl RegularPaperfolding<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Clone + Integer + CheckedDiv + CheckedAdd> Iterator for RegularPaperfolding<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.odd_part.next()?;

        if (n % 4) == 1 {
            Some(T::one())
        } else {
            Some(T::zero())
        }
    }
}

/// A general paperfolding sequence.
pub struct Paperfolding<T> {
    odd_part: OddPart<usize>,
    iter: Box<dyn Iterator<Item = T>>,
}

impl<T: Clone + Integer + CheckedDiv + CheckedAdd> Paperfolding<T> {
    /// The iterator should output only
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            odd_part: OddPart::new(),
            iter: Box::new(iter),
        }
    }
}

impl Paperfolding<BigInt> {
    pub fn new_big<I>(iter: I) -> Self
    where
        I: Iterator<Item = BigInt> + 'static,
    {
        Self::new(iter)
    }
}

impl<T: Clone + Integer + CheckedDiv + CheckedAdd> Iterator for Paperfolding<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.odd_part.next()?;

        if (n % 4) == 1 {
            self.iter.next()
        } else {
            match self.iter.next() {
                Some(b) => Some(T::one() - b),
                None => None,
            }
        }
    }
}

crate::check_sequences!(
    RegularPaperfolding::<u32>::new(), [1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0];
    Paperfolding::<u32>::new([1, 1, 1, 1, 1, 1, 1].into_iter()), [1, 1, 0, 1, 1, 0, 0];
    Paperfolding::<u32>::new([0, 0, 0, 0, 0, 0, 0].into_iter()), [0, 0, 1, 0, 0, 1, 1];
);
