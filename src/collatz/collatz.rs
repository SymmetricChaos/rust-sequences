use crate::{collatz::funcs::collatz, core::traits::Increment};
use num::{BigInt, CheckedAdd, CheckedDiv, CheckedMul, Integer};

/// The values of a Collatz sequence.
pub struct Collatz<T> {
    value: T,
}

impl<T: Clone + CheckedAdd + CheckedMul + CheckedDiv + Integer> Collatz<T> {
    /// Start a Collatz sequence from n.
    pub fn new(n: T) -> Self {
        Self { value: n }
    }
}

impl Collatz<BigInt> {
    /// Start a Collatz sequence from n.
    pub fn new_big<T>(n: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            value: BigInt::from(n),
        }
    }
}

impl<T: Clone + CheckedAdd + CheckedMul + CheckedDiv + Integer> Iterator for Collatz<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.value.clone();
        self.value = collatz(self.value.clone())?;
        Some(out)
    }
}

/// The odd values of a Collatz sequence.
pub struct ReducedCollatz<T> {
    value: T,
}

impl<T: Clone + CheckedAdd + CheckedMul + CheckedDiv + Integer> ReducedCollatz<T> {
    /// Start an odd Collatz sequence from n.
    pub fn new(n: T) -> Self {
        Self { value: n }
    }
}

impl ReducedCollatz<BigInt> {
    /// Start an odd Collatz sequence from n.
    pub fn new_big<T>(n: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            value: BigInt::from(n),
        }
    }
}

impl<T: Clone + CheckedAdd + CheckedMul + CheckedDiv + Integer> Iterator for ReducedCollatz<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.value.clone();

        self.value = self.value.checked_mul(&(T::one() + T::one() + T::one()))?;
        self.value = self.value.checked_add(&(T::one()))?;

        while self.value.is_even() {
            self.value = self.value.checked_div(&(T::one() + T::one()))?;
        }

        // if specialization were avaiable this would be faster than iteratively dividing out twos
        // self.value >>= self.value.trailing_zeros(); // Divide out all the twos.

        Some(out)
    }
}

/// Number of steps to reach 1 for a Collatz sequence starting at each positive natural number. It is not known if this sequence is defined for all inputs.
///
/// 0, 1, 7, 2, 5, 8, 16, 3, 19, 6, 14, 9, 9...
pub struct CollatzLength<T> {
    ctr: T,
}

impl<T: Clone + CheckedAdd + CheckedMul + CheckedDiv + Integer> CollatzLength<T> {
    pub fn new() -> Self {
        Self { ctr: T::zero() }
    }
}

impl CollatzLength<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Clone + CheckedAdd + CheckedMul + CheckedDiv + Integer> Iterator for CollatzLength<T> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.incr();
        let mut steps = 0;

        let mut val = self.ctr.clone();
        while !val.is_one() {
            if val.is_even() {
                val = val.checked_div(&(T::one() + T::one()))?;
            } else {
                val = val.checked_mul(&(T::one() + T::one() + T::one()))?;
                val = val.checked_add(&(T::one()))?;
            }
            steps.incr()?;
        }

        Some(steps)
    }
}

crate::check_sequences!(
    Collatz::new_big(19), [19, 58, 29, 88, 44, 22, 11, 34, 17, 52];
    Collatz::new_big(27), [27, 82, 41, 124, 62, 31, 94, 47, 142, 71];
    ReducedCollatz::new_big(27), [27, 41, 31, 47, 71, 107, 161, 121, 91, 137];
    ReducedCollatz::new(27), [27, 41, 31, 47, 71, 107, 161, 121, 91, 137];
    Collatz::new_big(-5), [-5, -14, -7, -20, -10, -5, -14, -7, -20, -10];
    CollatzLength::new_big(), [0, 1, 7, 2, 5, 8, 16, 3, 19, 6, 14, 9, 9, 17, 17, 4, 12, 20, 20, 7, 7, 15, 15, 10, 23, 10, 111, 18, 18, 18, 106, 5, 26, 13, 13, 21, 21, 21, 34, 8, 109, 8, 29, 16, 16, 16, 104, 11, 24, 24, 24, 11, 11, 112, 112, 19, 32, 19, 32, 19, 19, 107, 107, 6, 27, 27, 27, 14, 14, 14, 102, 22];
);
