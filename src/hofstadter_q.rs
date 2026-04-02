use num::BigInt;
use std::marker::PhantomData;

/// Hofstadter's Q-sequence. A doubly recursive sequence in which the two previous terms determine the terms added together to determine the next.
///
/// 1, 1, 2, 3, 3, 4, 5, 5, 6, 6, 6, 8, 8, 8, 10, 9, 10, 11, 11...
pub struct HofstadterQ<T> {
    terms: Vec<usize>,
    ctr: usize,
    phantom: PhantomData<T>,
}

impl<T> HofstadterQ<T>
where
    T: TryFrom<usize>,
{
    /// All internal calculations are done using usize and converted before being returned.
    pub fn new() -> Self {
        Self {
            terms: vec![1, 1],
            ctr: 1,
            phantom: PhantomData,
        }
    }
}

impl HofstadterQ<BigInt> {
    /// All internal calculations are done using usize and converted before being returned.
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T> Iterator for HofstadterQ<T>
where
    T: TryFrom<usize>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr = self.ctr.checked_add(1)?;
        let out = self.terms[self.ctr - 2];

        let n = self.ctr;

        let a = self.terms[n - self.terms[n - 1]];
        let b = self.terms[n - self.terms[n - 2]];

        self.terms.push(a.checked_add(b)?);

        T::try_from(out).ok()
    }
}

crate::check_sequences!(
    HofstadterQ::<usize>::new(), [1, 1, 2, 3, 3, 4, 5, 5, 6, 6, 6, 8, 8, 8, 10, 9, 10, 11, 11, 12, 12, 12, 12, 16];
);
