use crate::Number;
use num::{BigInt, Integer, integer::Roots};

/// Terms of the juggler sequences. If a term is even the next is the floor of its square root. If a term is odd the next term is the floor of the square root of its cube.
///
/// ```text
/// n = 3
/// 3, 5, 11, 36, 6, 2, 1
///
/// n = 9
/// 9, 27, 140, 11, 36, 6, 2, 1
/// ```
pub struct Juggler<T> {
    n: T,
}

impl Juggler<Number> {
    pub fn new(n: Number) -> Self {
        Self { n }
    }
}

impl Juggler<BigInt> {
    pub fn new_big<G>(n: G) -> Self
    where
        BigInt: From<G>,
    {
        Self { n: BigInt::from(n) }
    }
}

impl Iterator for Juggler<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.n.clone();

        if self.n.is_even() {
            self.n = self.n.sqrt();
        } else {
            // This will end due to overflow much earlier than strictly necessary
            // Newton's method may avoid this issue
            self.n = self.n.checked_pow(3)?.sqrt();
        }

        Some(out)
    }
}

impl Iterator for Juggler<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.n.clone();

        if self.n.is_even() {
            self.n = self.n.sqrt();
        } else {
            self.n = self.n.pow(3).sqrt();
        }

        Some(out)
    }
}

crate::check_sequences!(
    Juggler::new(3), [3, 5, 11, 36, 6, 2, 1];
    Juggler::new(9), [9, 27, 140, 11, 36, 6, 2, 1];
);
