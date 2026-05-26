use crate::utils::collatz::{collatz, reduced_collatz};
use num::{BigInt, CheckedAdd, CheckedMul, Integer};

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
    value: T,
}

impl<T: Clone + CheckedAdd + CheckedMul + Integer> Collatz<T> {
    pub fn new(n: T) -> Self {
        Self { value: n }
    }
}

impl Collatz<BigInt> {
    pub fn new_big<T>(n: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            value: BigInt::from(n),
        }
    }
}

impl<T: Clone + CheckedAdd + CheckedMul + Integer> Iterator for Collatz<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.value.clone();
        self.value = match collatz(self.value.clone()) {
            Some(n) => n,
            None => return Some(out),
        };
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
    value: T,
}

impl<T: Clone> ReducedCollatz<T> {
    pub fn new(n: T) -> Self {
        Self { value: n }
    }
}

impl ReducedCollatz<BigInt> {
    pub fn new_big<T>(n: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            value: BigInt::from(n),
        }
    }
}

impl<T: Clone + CheckedAdd + CheckedMul + Integer> Iterator for ReducedCollatz<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.value.clone();
        self.value = match reduced_collatz(self.value.clone()) {
            Some(n) => n,
            None => return Some(out),
        };
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
