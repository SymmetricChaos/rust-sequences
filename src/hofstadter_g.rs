use num::BigInt;
use std::marker::PhantomData;

/// Hofstadter's G-sequence. A multiply recursive sequence starting with 0. a(n) = n - a(a(n-1))
///
/// 0, 1, 1, 2, 3, 3, 4, 4, 5, 6, 6, 7, 8, 8, 9...
pub struct HofstadterG<T> {
    terms: Vec<usize>,
    ctr: usize,
    _phantom: PhantomData<T>,
}

impl<T> HofstadterG<T>
where
    T: TryFrom<usize>,
{
    /// All internal calculations are done using usize and converted before being returned.
    pub fn new() -> Self {
        Self {
            terms: vec![0],
            ctr: 0,
            _phantom: PhantomData,
        }
    }
}

impl HofstadterG<BigInt> {
    /// All internal calculations are done using usize and converted before being returned.
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T> Iterator for HofstadterG<T>
where
    T: TryFrom<usize>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.terms.last()?.clone();

        self.ctr = self.ctr.checked_add(1)?;
        self.terms.push(self.ctr - self.terms[out]);

        T::try_from(out).ok()
    }
}

crate::check_sequences!(
    HofstadterG::<usize>::new(), [0, 1, 1, 2, 3, 3, 4, 4, 5, 6, 6, 7, 8, 8, 9, 9, 10, 11, 11, 12, 12, 13, 14, 14, 15, 16, 16, 17, 17, 18, 19, 19, 20, 21, 21, 22, 22, 23, 24, 24, 25, 25, 26, 27];
);
