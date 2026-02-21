use num::{BigInt, CheckedMul, One};

/// The powers of n.
pub struct Powers<T> {
    value: T,
    n: T,
}

impl<T: CheckedMul + Clone + One> Powers<T> {
    pub fn new(n: T) -> Self {
        Self { value: T::one(), n }
    }
}

impl Powers<BigInt> {
    pub fn new_big<T: Clone>(n: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            value: BigInt::one(),
            n: BigInt::from(n),
        }
    }
}

impl<T: CheckedMul + Clone> Iterator for Powers<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.value.clone();
        self.value = self.value.checked_mul(&self.n)?;
        Some(out)
    }

    // Should be slightly faster than iteration with .next()
    // fn nth(&mut self, n: usize) -> Option<Self::Item> {
    //     self.value *= self.n.pow(n.try_into().unwrap());
    //     let out = self.value.clone();
    //     self.value *= &self.n;
    //     Some(out)
    // }
}

crate::print_sequences!(
    Powers::new_big(3), 5, 10;
    Powers::new(crate::core::modular_int::ModInt::<17>::new(2)), 0, 20;
);

crate::check_sequences!(
    Powers::new_big(-2), [1, -2, 4, -8, 16, -32, 64, -128, 256, -512];
    Powers::new_big(1), [1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    Powers::new_big(2), [1, 2, 4, 8, 16, 32, 64, 128, 256, 512];
    Powers::new_big(3), [1, 3, 9, 27, 81, 243, 729, 2187, 6561, 19683];
    Powers::new_big(4), [1, 4, 16, 64, 256, 1024, 4096, 16384, 65536, 262144];
);
