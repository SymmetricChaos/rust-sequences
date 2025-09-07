use num::{BigInt, One};

/// The generalized polygonal numbers with selectable order.
pub struct GeneralizedPolygonal {
    integers: crate::core::Integers,
    k: BigInt,
}

impl GeneralizedPolygonal {
    pub fn new<T: One>(k: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            integers: crate::core::Integers::new(),
            k: BigInt::from(k),
        }
    }
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

    // Slight optimization to .nth() also make .skip() faster.
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        for _ in 0..n {
            self.integers.next();
        }
        let out = ((&self.k - 2) * n * n - (&self.k - 4) * n) / 2;
        Some(out)
    }
}

crate::print_values!(
    GeneralizedPolygonal::new(1), 0, 10;
    GeneralizedPolygonal::new(2), 0, 10;
    GeneralizedPolygonal::new(3), 0, 10;
    GeneralizedPolygonal::new(4), 0, 10;
    GeneralizedPolygonal::new(5), 0, 10;
    GeneralizedPolygonal::new(6), 0, 10;
);

crate::check_sequences!(
    GeneralizedPolygonal::new(5), 0, 10, [0, 1, 2, 5, 7, 12, 15, 22, 26, 35]; // Generalized pentagonal numbers are particularly important
);
