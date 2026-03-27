use num::BigInt;

use crate::core::integer::Integers;

/// The generalized polygonal numbers with selectable order. Extends the domain of the polygonal numbers to all integers.
pub struct GeneralizedPolygonal {
    integers: Integers<BigInt>,
    k: BigInt,
}

impl GeneralizedPolygonal {
    /// The order, k, may be any integer but some have well known names.
    ///
    /// k = 2 produces the natural numbers
    ///
    /// k = 3 produces the triangular numbers
    ///
    /// k = 4 produces the square numbers
    ///
    /// and so on for higher orders
    pub fn new_big<T>(k: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            integers: Integers::new_big(),
            k: BigInt::from(k),
        }
    }

    /// The order, k, may be any integer but some have well known names.
    ///
    /// k = 2 produces the natural numbers
    ///
    /// k = 3 produces the triangular numbers
    ///
    /// k = 4 produces the square numbers
    ///
    /// and so on for higher orders
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
}

crate::check_iteration_times!(
    GeneralizedPolygonal::new_big(5), 700_000;
);

crate::print_sequences!(
    GeneralizedPolygonal::new_big(-1), 10;
    GeneralizedPolygonal::new_big(0), 10;
    GeneralizedPolygonal::new_big(1), 10;
    GeneralizedPolygonal::new_big(2), 10;
    GeneralizedPolygonal::new_big(3), 10;
);

crate::check_sequences!(
    GeneralizedPolygonal::new_big(5), [0, 1, 2, 5, 7, 12, 15, 22, 26, 35]; // Generalized pentagonal numbers are particularly important
);
