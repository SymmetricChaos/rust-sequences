use num::{BigInt, CheckedAdd, One};

/// The values of Narayana's cows sequence. From a word problem in which a mature cow produces one cow per year and a cow takes four years to mature.
///
/// 1, 1, 1, 2, 3, 4, 6, 9, 13, 19...
pub struct NarayanasCows<T> {
    a: T,
    b: T,
    c: T,
}

impl<T: One> NarayanasCows<T> {
    pub fn new() -> Self {
        Self {
            a: T::one(),
            b: T::one(),
            c: T::one(),
        }
    }
}

impl NarayanasCows<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Clone + CheckedAdd> Iterator for NarayanasCows<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();

        let new = self.a.clone().checked_add(&self.c)?;
        self.a = self.b.clone();
        self.b = self.c.clone();
        self.c = new;

        Some(out)
    }
}

crate::check_sequences!(
    NarayanasCows::<u16>::new(), [1, 1, 1, 2, 3, 4, 6, 9, 13, 19, 28, 41, 60, 88];
);
