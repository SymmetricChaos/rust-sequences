use num::BigInt;
use std::marker::PhantomData;

/// Hofstader's married sequences.
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
    pub fn new() -> Self {
        Self {
            ctr: 0,
            a: vec![1],
            b: vec![0],
            _phantom: PhantomData,
        }
    }

    pub fn female() -> impl Iterator<Item = T> {
        Self::new().map(|x| x.0)
    }

    pub fn male() -> impl Iterator<Item = T> {
        Self::new().map(|x| x.1)
    }
}

impl Married<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }

    pub fn female_big() -> impl Iterator<Item = BigInt> {
        Self::new().map(|x| x.0)
    }

    pub fn male_big() -> impl Iterator<Item = BigInt> {
        Self::new().map(|x| x.1)
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
