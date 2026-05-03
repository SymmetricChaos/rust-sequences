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
        Self::new(BigInt::from(n))
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
    Powers::new_big(3), skip 5, 10;
    Powers::new(crate::utils::modular_int::ModInt::<17>::new(2)), 20;
);

crate::check_sequences!(
    Powers::new_big(-2), [1, -2, 4, -8, 16, -32, 64, -128, 256, -512];
    Powers::new_big(1), [1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    Powers::new_big(2), [1_u64, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216, 33554432, 67108864, 134217728, 268435456, 536870912, 1073741824, 2147483648, 4294967296, 8589934592, 17179869184];
    Powers::new_big(3), [1_u64, 3, 9, 27, 81, 243, 729, 2187, 6561, 19683, 59049, 177147, 531441, 1594323, 4782969, 14348907, 43046721, 129140163, 387420489, 1162261467, 3486784401, 10460353203, 31381059609, 94143178827, 282429536481, 847288609443, 2541865828329, 7625597484987];
);
