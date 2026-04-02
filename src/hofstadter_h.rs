use num::BigInt;
use std::marker::PhantomData;

/// Hofstadter's H-sequence. A multiply recursive sequence starting with 0. a(n) = n - a(a(a(n-1)))
///
/// 0, 1, 1, 2, 3, 4, 4, 5, 5, 6, 7, 7, 8, 9, 10...
pub struct HofstadterH<T> {
    terms: Vec<usize>,
    ctr: usize,
    phantom: PhantomData<T>,
}

impl<T> HofstadterH<T>
where
    T: TryFrom<usize>,
{
    /// All internal calculations are done using usize and converted before being returned.
    pub fn new() -> Self {
        Self {
            terms: vec![0],
            ctr: 0,
            phantom: PhantomData,
        }
    }
}

impl HofstadterH<BigInt> {
    /// All internal calculations are done using usize and converted before being returned.
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T> Iterator for HofstadterH<T>
where
    T: TryFrom<usize>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.terms.last()?.clone();

        self.ctr = self.ctr.checked_add(1)?;
        self.terms.push(self.ctr - self.terms[self.terms[out]]);

        T::try_from(out).ok()
    }
}

crate::check_sequences!(
    HofstadterH::<usize>::new(), [0, 1, 1, 2, 3, 4, 4, 5, 5, 6, 7, 7, 8, 9, 10, 10, 11, 12, 13, 13, 14, 14, 15, 16, 17, 17, 18, 18, 19, 20, 20, 21, 22, 23, 23, 24, 24, 25, 26, 26, 27, 28];
);
