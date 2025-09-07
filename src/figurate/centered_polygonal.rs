use num::{BigInt, One};

use crate::figurate::polygonal::Polygonal;

/// The centered polygonal numbers with selectable order
pub struct CenteredPolygonal {
    k: BigInt,
    polygonal: Polygonal,
}

impl CenteredPolygonal {
    /// k = 3 -> Centered triangular numbers
    /// 1, 4, 10, 19, 31, 46, 64, 85, 109, 136...
    /// k = 4 -> Centered square numbers
    /// 1, 5, 13, 25, 41, 61, 85, 113, 145, 181...
    /// Lower values of k are consistent but do not have standard names but resemble Hogben's central polygonal numbers
    pub fn new<T: One>(k: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            k: BigInt::from(k),
            polygonal: Polygonal::new(T::one()),
        }
    }

    // /// The nth centered polygonal number of order k
    // /// Panics if n or k is negative.
    // pub fn nth<T>(k: T, n: T) -> BigInt
    // where
    //     BigInt: From<T>,
    // {
    //     let n = BigInt::from(n);
    //     let k = BigInt::from(k);
    //     (Polygonal::nth(n.clone(), k.clone()) * k) + 1
    // }
}

impl Iterator for CenteredPolygonal {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.polygonal.next()? * &self.k + 1;
        Some(out)
    }
}

// These low order values are the same as two versions of Hogben's central polygonal numbers
crate::print_a_few!(
    CenteredPolygonal::new(1), 0, 10;
    CenteredPolygonal::new(2), 0, 10;
);

crate::check_sequences!(
    CenteredPolygonal::new(3), 0, 10, [1, 4, 10, 19, 31, 46, 64, 85, 109, 136];
    CenteredPolygonal::new(4), 0, 10, [1, 5, 13, 25, 41, 61, 85, 113, 145, 181];
    CenteredPolygonal::new(5), 0, 10, [1, 6, 16, 31, 51, 76, 106, 141, 181, 226];
    CenteredPolygonal::new(6), 0, 10, [1, 7, 19, 37, 61, 91, 127, 169, 217, 271];
);
