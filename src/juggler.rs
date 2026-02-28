use num::{BigInt, CheckedMul, Integer, integer::Roots};

/// A juggler sequence.
pub struct Juggler<T> {
    n: T,
}

impl<T> Juggler<T> {
    pub fn new(n: T) -> Self {
        Self { n }
    }
}

impl Juggler<BigInt> {
    pub fn new_big<T>(n: T) -> Self
    where
        BigInt: From<T>,
    {
        Self { n: BigInt::from(n) }
    }
}

impl<T: Integer + Roots + Clone + CheckedMul> Iterator for Juggler<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.n.clone();

        if self.n.is_even() {
            self.n = self.n.sqrt();
        } else {
            // This will end due to overflow much earlier than strictly necessary
            self.n = self.n.checked_mul(&self.n)?.checked_mul(&self.n)?.sqrt();
        }

        Some(out)
    }
}

crate::check_sequences!(
    Juggler::<u32>::new(3), [3, 5, 11, 36, 6, 2, 1];
    Juggler::<i16>::new(9), [9, 27, 140, 11, 36, 6, 2, 1];
);
