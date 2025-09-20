use num::{BigInt, One, PrimInt, Signed};

/// The generalized polygonal numbers with selectable order.
pub struct GeneralizedPolygonal {
    integers: crate::core::Integer<BigInt>,
    k: BigInt,
}

impl GeneralizedPolygonal {
    /// The order, k, is the number of sides of the polygon.
    /// k = 2 produces the natural numbers
    /// k = 3 produces the triangular numbers
    /// k = 4 produces the square numbers
    /// and so on for higher orders
    /// Lower values of k are allowed but do not have standard names.
    pub fn new_big<T: One>(k: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            integers: crate::core::Integer::new_big(),
            k: BigInt::from(k),
        }
    }
    /// The order, k, is the number of sides of the polygon.
    /// k = 2 produces the natural numbers
    /// k = 3 produces the triangular numbers
    /// k = 4 produces the square numbers
    /// and so on for higher orders
    /// Lower values of k are allowed but do not have standard names.
    pub fn nth<T>(n: T, k: T) -> BigInt
    where
        BigInt: From<T>,
    {
        let n = &BigInt::from(n);
        let k = &BigInt::from(k);
        ((k - 2) * n * n - (k - 4) * n) / 2
    }
}

impl Iterator for GeneralizedPolygonal {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let n = &self.integers.next()?;
        let out = ((&self.k - 2) * n * n - (&self.k - 4) * n) / 2;
        Some(out)
    }

    // // Slight optimization to .nth() also make .skip() faster.
    // fn nth(&mut self, n: usize) -> Option<Self::Item> {
    //     for _ in 0..n {
    //         self.integers.next();
    //     }
    //     let out = ((&self.k - 2) * n * n - (&self.k - 4) * n) / 2;
    //     Some(out)
    // }
}

/// The generalized pentagonal numbers
pub struct GeneralizedPentagonal {
    integers: crate::core::Integer<BigInt>,
}

impl GeneralizedPentagonal {
    pub fn new_big() -> Self {
        Self {
            integers: crate::core::Integer::new_big(),
        }
    }

    pub fn nth<T>(n: T) -> BigInt
    where
        BigInt: From<T>,
    {
        let n = &BigInt::from(n);
        (3 * n * n - n) / 2
    }
}

impl Iterator for GeneralizedPentagonal {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let n = &self.integers.next()?;
        let out = (3 * n * n - n) / 2;
        Some(out)
    }
}

/// The generalized pentagonal numbers
/// T is restricted to Signed due to how values are generated from IntegerGeneric
pub struct GeneralizedPentagonalGeneric<T> {
    integers: crate::core::Integer<T>,
}

impl<T: PrimInt + Signed> GeneralizedPentagonalGeneric<T> {
    pub fn new() -> Self {
        Self {
            integers: crate::core::Integer::<T>::new(),
        }
    }

    pub fn nth(n: T) -> Option<T> {
        let three = T::one() + T::one() + T::one();
        Some(three.checked_mul(&n)?.checked_mul(&n)?.checked_sub(&n)? >> 1)
    }
}

impl<T: PrimInt + Signed> Iterator for GeneralizedPentagonalGeneric<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let three = T::one() + T::one() + T::one();
        let n = self.integers.next()?;
        let out = three.checked_mul(&n)?.checked_mul(&n)?.checked_sub(&n)? >> 1;
        Some(out)
    }
}

crate::check_iteration_times!(
    GeneralizedPolygonal::new_big(5), 700_000;
    GeneralizedPentagonal::new_big(), 1_500_000;
    GeneralizedPentagonalGeneric::<i64>::new(), 35_000_000;
);

crate::print_values!(
    GeneralizedPolygonal::new_big(1), 0, 10;
    GeneralizedPolygonal::new_big(2), 0, 10;
    GeneralizedPolygonal::new_big(3), 0, 10;
    GeneralizedPolygonal::new_big(4), 0, 10;
    GeneralizedPolygonal::new_big(5), 0, 10;
    GeneralizedPolygonal::new_big(6), 0, 10;
);

crate::check_sequences!(
    GeneralizedPolygonal::new_big(5), 0, 10, [0, 1, 2, 5, 7, 12, 15, 22, 26, 35]; // Generalized pentagonal numbers are particularly important
    GeneralizedPentagonal::new_big(), 0, 10, [0, 1, 2, 5, 7, 12, 15, 22, 26, 35];
);
