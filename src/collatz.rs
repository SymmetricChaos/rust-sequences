use num::{BigInt, CheckedAdd, CheckedDiv, CheckedMul, Integer};

/// The values of the Collatz (aka hailstone) sequences. If a term n the next is 3n+1 if n is odd and n/2 if n is even.
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
        if out.is_even() {
            self.value = self.value.checked_div(&(T::one() + T::one()))?;
        } else {
            self.value = self.value.checked_mul(&(T::one() + T::one() + T::one()))?;
            self.value = self.value.checked_add(&(T::one()))?;
        }
        Some(out)
    }
}

/// The odd values of the Collatz (aka hailstone) sequences. This is also called the shortcut version of the sequence.
pub struct CollatzOdd<T> {
    value: T,
}

impl<T: Clone + CheckedAdd + CheckedMul + CheckedDiv + Integer> CollatzOdd<T> {
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

/// The values of the generalized Collatz (aka hailstone) sequences. The term after n is an+b if n is odd or n/2 if n is even.
pub struct CollatzGeneral<T> {
    n: T,
    a: T,
    b: T,
}

impl<T: Clone + CheckedAdd + CheckedMul + CheckedDiv + Integer> CollatzGeneral<T> {
    /// Start a generalized Collatz sequence from n. The term after n is an+b if n is odd or n/2 if n is even.
    pub fn new(n: T, a: T, b: T) -> Self {
        Self { n, a, b }
    }
}

impl CollatzGeneral<BigInt> {
    /// Start a generalized Collatz sequence from n. The term after n is an+b if n is odd or n/2 if n is even.
    pub fn new_big<T>(n: T, a: T, b: T) -> Self
    where
        BigInt: From<T>,
    {
        Self::new(BigInt::from(n), BigInt::from(a), BigInt::from(b))
    }
}

impl<T: Clone + CheckedAdd + CheckedMul + CheckedDiv + Integer> Iterator for CollatzGeneral<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.n.clone();
        if out.is_even() {
            self.n = self.n.checked_div(&(T::one() + T::one()))?;
        } else {
            self.n = self.n.checked_mul(&self.a)?;
            self.n = self.n.checked_add(&self.b)?;
        }
        Some(out)
    }
}

crate::check_sequences!(
    Collatz::new(19), [19, 58, 29, 88, 44, 22, 11, 34, 17, 52];
    Collatz::new_big(19), [19, 58, 29, 88, 44, 22, 11, 34, 17, 52];
    Collatz::new_big(27), [27, 82, 41, 124, 62, 31, 94, 47, 142, 71];
    CollatzOdd::new_big(27), [27, 41, 31, 47, 71, 107, 161, 121, 91, 137];
    CollatzOdd::new(27), [27, 41, 31, 47, 71, 107, 161, 121, 91, 137];
    Collatz::new_big(-5), [-5, -14, -7, -20, -10, -5, -14, -7, -20, -10];
);
