use num::{BigInt, One};

use crate::figurate::polygonal::Polygonal;

/// The centered polygonal numbers with selectable order
pub struct CenteredPolygonal {
    k: BigInt,
    polygonal: Polygonal,
}

impl CenteredPolygonal {
    /// k = 3 produces the centered triangular numbers
    /// k = 4 produces the centered square numbers
    /// Lower values of k are related to Hogben's central polygonal numbers but do not have standard names.
    pub fn new_big<T: One>(k: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            k: BigInt::from(k),
            polygonal: Polygonal::new_big::<i32>(3),
        }
    }

    /// k = 3 produces the centered triangular numbers
    /// k = 4 produces the centered square numbers
    /// Lower values of k are related to Hogben's central polygonal numbers but do not have standard names.
    pub fn nth<T>(n: T, k: T) -> BigInt
    where
        BigInt: From<T>,
    {
        let k = &BigInt::from(k);
        let n = &BigInt::from(n);
        (k * n * (n - 1)) / 2 + 1
    }
}

impl Iterator for CenteredPolygonal {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.polygonal.next()? * &self.k + 1;
        Some(out)
    }
}

// These low order values are the same as two versions of Hogben's central polygonal numbers
crate::print_values!(
    CenteredPolygonal::new_big(1), 0, 10;
    CenteredPolygonal::new_big(2), 0, 10;
);

crate::check_sequences!(
    CenteredPolygonal::new_big(3), 0, 10, [1, 4, 10, 19, 31, 46, 64, 85, 109, 136];
    CenteredPolygonal::new_big(4), 0, 10, [1, 5, 13, 25, 41, 61, 85, 113, 145, 181];
    CenteredPolygonal::new_big(5), 0, 10, [1, 6, 16, 31, 51, 76, 106, 141, 181, 226];
    CenteredPolygonal::new_big(6), 0, 10, [1, 7, 19, 37, 61, 91, 127, 169, 217, 271];
);
