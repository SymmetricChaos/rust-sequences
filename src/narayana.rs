use num::{BigInt, CheckedAdd, One};

/// The values of Narayana's cow sequence
pub struct Narayana<T> {
    a: T,
    b: T,
    c: T,
}

impl<T: One> Narayana<T> {
    pub fn new() -> Self {
        Self {
            a: T::one(),
            b: T::one(),
            c: T::one(),
        }
    }
}

impl Narayana<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Clone + CheckedAdd> Iterator for Narayana<T> {
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
    Narayana::<u16>::new(), [1, 1, 1, 2, 3, 4, 6, 9, 13, 19, 28, 41, 60, 88];
);
