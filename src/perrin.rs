use num::{BigInt, CheckedAdd, Integer};

pub struct Perrin<T> {
    n0: T,
    n1: T,
    n2: T,
}

impl<T: Integer> Perrin<T> {
    pub fn new() -> Self {
        Self {
            n0: T::one() + T::one() + T::one(),
            n1: T::zero(),
            n2: T::one() + T::one(),
        }
    }
}

impl Perrin<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Integer + Clone + CheckedAdd> Iterator for Perrin<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.n0.clone();

        let n3 = self.n0.checked_add(&self.n1)?;
        self.n0 = self.n1.clone();
        self.n1 = self.n2.clone();
        self.n2 = n3;

        Some(out)
    }
}

crate::check_sequences!(
    Perrin::<i32>::new(), [3, 0, 2, 3, 2, 5, 5, 7, 10, 12, 17, 22];
);
