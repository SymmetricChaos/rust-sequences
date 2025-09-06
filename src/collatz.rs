use num::{BigInt, Integer};

/// The values of the Collatz (aka hailstone) sequences.
pub struct Collatz {
    value: BigInt,
}

impl Collatz {
    /// Start a Collatz sequence from n.
    pub fn new<T>(n: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            value: BigInt::from(n),
        }
    }
}

impl Iterator for Collatz {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.value.clone();
        if out.is_even() {
            self.value /= 2;
        } else {
            self.value *= 3;
            self.value += 1;
        }
        Some(out)
    }
}

/// The odd values of the Collatz (aka hailstone) sequences. In the usual Collatz sequences powers of two are divided out one step at a time. In these they are all divided out in one step.
pub struct CollatzOdd {
    value: BigInt,
}

impl CollatzOdd {
    /// Start an odd Collatz sequence from n.
    pub fn new<T>(n: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            value: BigInt::from(n),
        }
    }
}

impl Iterator for CollatzOdd {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.value.clone();

        self.value *= 3;
        self.value += 1;
        self.value >>= self.value.trailing_zeros().unwrap(); // Divide out all the twos.

        Some(out)
    }
}

crate::check_sequences!(
    Collatz::new(19), 0, 10, [19, 58, 29, 88, 44, 22, 11, 34, 17, 52];
    Collatz::new(27), 0, 10, [27, 82, 41, 124, 62, 31, 94, 47, 142, 71];
    CollatzOdd::new(27), 0, 10, [27, 41, 31, 47, 71, 107, 161, 121, 91, 137];
    Collatz::new(-5), 0, 10, [-5, -14, -7, -20, -10, -5, -14, -7, -20, -10];
);
