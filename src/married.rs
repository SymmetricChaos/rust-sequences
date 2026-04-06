use num::BigInt;
use std::marker::PhantomData;

/// Hofstadter's married sequences aka the male-female sequences. Defined by two entanged recurrences:
///
/// a(n) = n - b(a(n-1)), a(0) = 1
///
/// b(n) = n - a(b(n-1)), b(0) = 0
///
/// a sequence (female): 1, 1, 2, 2, 3, 3, 4, 5, 5, 6, 6...
///
/// b sequence (male):   0, 0, 1, 2, 2, 3, 4, 4, 5, 6, 6...
pub struct Married<T> {
    ctr: usize,
    a: Vec<usize>,
    b: Vec<usize>,
    _phantom: PhantomData<T>,
}

impl<T> Married<T>
where
    T: TryFrom<usize>,
{
    /// Returns both sequences simultaneously.
    pub fn new() -> Self {
        Self {
            ctr: 0,
            a: vec![1],
            b: vec![0],
            _phantom: PhantomData,
        }
    }

    /// Returns only the female (a) sequence.
    pub fn female() -> impl Iterator<Item = T> {
        Self::new().map(|x| x.0)
    }

    /// Returns only the male (b) sequence.
    pub fn male() -> impl Iterator<Item = T> {
        Self::new().map(|x| x.1)
    }
}

impl Married<BigInt> {
    /// Returns both sequences simultaneously.
    pub fn new_big() -> Self {
        Self::new()
    }

    /// Returns only the female (a) sequence.
    pub fn female_big() -> impl Iterator<Item = BigInt> {
        Self::female()
    }

    /// Returns only the male (b) sequence.
    pub fn male_big() -> impl Iterator<Item = BigInt> {
        Self::male()
    }
}

impl<T> Iterator for Married<T>
where
    T: TryFrom<usize>,
{
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let out = (
            T::try_from(self.a[self.ctr]).ok()?,
            T::try_from(self.b[self.ctr]).ok()?,
        );

        self.b.push(self.ctr + 1 - self.a[self.b[self.ctr]]);
        self.a.push(self.ctr + 1 - self.b[self.a[self.ctr]]);

        self.ctr += 1;

        Some(out)
    }
}

crate::check_sequences!(
    Married::<usize>::female(), [1, 1, 2, 2, 3, 3, 4, 5, 5, 6, 6, 7, 8, 8, 9, 9, 10, 11, 11, 12, 13, 13, 14, 14, 15, 16, 16, 17, 17, 18, 19, 19, 20, 21, 21, 22, 22, 23, 24, 24, 25, 25, 26, 27];
    Married::<usize>::male(),   [0, 0, 1, 2, 2, 3, 4, 4, 5, 6, 6, 7, 7, 8, 9, 9, 10, 11, 11, 12, 12, 13, 14, 14, 15, 16, 16, 17, 17, 18, 19, 19, 20, 20, 21, 22, 22, 23, 24, 24, 25, 25, 26, 27];
);
