use crate::{
    Number,
    utils::collatz::{collatz, reduced_collatz},
};
use num::{BigInt, Integer};

/// The values of a Collatz sequence.
///
/// ```text
/// f(n) = 3n+1 for odd n
///      = n/2  for even n
///
/// n = 19
/// 19, 58, 29, 88, 44, 22, 11, 34, 17, 52...
///
/// n = 27
/// 27, 82, 41, 124, 62, 31, 94, 47, 142, 71...
/// ```
pub struct Collatz<T> {
    n: T,
}

impl Collatz<Number> {
    pub fn new(n: Number) -> Self {
        Self { n }
    }
}

impl Collatz<BigInt> {
    pub fn new_big<T>(n: T) -> Self
    where
        BigInt: From<T>,
    {
        Self { n: BigInt::from(n) }
    }
}

impl Iterator for Collatz<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.n.clone();
        self.n = match collatz(self.n) {
            Some(n) => n,
            None => return Some(out),
        };
        Some(out)
    }
}

impl Iterator for Collatz<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.n.clone();
        if self.n.is_even() {
            self.n /= 2
        } else {
            self.n = (&self.n * 3) + 1
        }
        Some(out)
    }
}

/// The odd values of a Collatz sequence.
///
/// ```text
/// n = 19
/// 19, 29, 11, 17, 13, 5, 1, 1, 1, 1...
///
/// n = 27
/// 27, 41, 31, 47, 71, 107, 161, 121, 91, 137...
/// ```
pub struct ReducedCollatz<T> {
    n: T,
}

impl ReducedCollatz<Number> {
    pub fn new(n: Number) -> Self {
        Self { n }
    }
}

impl ReducedCollatz<BigInt> {
    pub fn new_big<T>(n: T) -> Self
    where
        BigInt: From<T>,
    {
        Self { n: BigInt::from(n) }
    }
}

impl Iterator for ReducedCollatz<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.n.clone();
        self.n = match reduced_collatz(self.n) {
            Some(n) => n,
            None => return Some(out),
        };
        Some(out)
    }
}

impl Iterator for ReducedCollatz<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.n.clone();
        self.n = (&self.n * 3) + 1;
        while self.n.is_even() {
            self.n /= 2;
        }
        Some(out)
    }
}

crate::check_sequences!(
    Collatz::new_big(19), [19, 58, 29, 88, 44, 22, 11, 34, 17, 52];
    Collatz::new_big(27), [27, 82, 41, 124, 62, 31, 94, 47, 142, 71];
    ReducedCollatz::new_big(19), [19, 29, 11, 17, 13, 5, 1, 1, 1, 1];
    ReducedCollatz::new_big(27), [27, 41, 31, 47, 71, 107, 161, 121, 91, 137];
    ReducedCollatz::new(27), [27, 41, 31, 47, 71, 107, 161, 121, 91, 137];
    Collatz::new_big(-5), [-5, -14, -7, -20, -10, -5, -14, -7, -20, -10];
);
