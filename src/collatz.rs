use num::{BigInt, CheckedAdd, CheckedDiv, CheckedMul, Integer};

/// The values of the Collatz (aka hailstone) sequences.
pub struct Collatz<T> {
    value: T,
}

impl<T> Collatz<T> {
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
        if out.is_even() {
            self.value = self.value.checked_div(&(T::one() + T::one()))?;
        } else {
            self.value = self.value.checked_mul(&(T::one() + T::one() + T::one()))?;
            self.value = self.value.checked_add(&(T::one()))?;
        }
        Some(out)
    }
}

/// The odd values of the Collatz (aka hailstone) sequences. In the usual Collatz sequences powers of two are divided out one step at a time. In these they are all divided out in one step.
pub struct CollatzOdd<T> {
    value: T,
}

impl<T> CollatzOdd<T> {
    /// Start an odd Collatz sequence from n.
    pub fn new(n: T) -> Self {
        Self { value: n }
    }
}

impl CollatzOdd<BigInt> {
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

impl<T: Clone + CheckedAdd + CheckedMul + CheckedDiv + Integer> Iterator for CollatzOdd<T> {
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

crate::check_sequences!(
    Collatz::new(19), 0, 10, [19, 58, 29, 88, 44, 22, 11, 34, 17, 52];
    Collatz::new_big(19), 0, 10, [19, 58, 29, 88, 44, 22, 11, 34, 17, 52];
    Collatz::new_big(27), 0, 10, [27, 82, 41, 124, 62, 31, 94, 47, 142, 71];
    CollatzOdd::new_big(27), 0, 10, [27, 41, 31, 47, 71, 107, 161, 121, 91, 137];
    Collatz::new_big(-5), 0, 10, [-5, -14, -7, -20, -10, -5, -14, -7, -20, -10];
);
