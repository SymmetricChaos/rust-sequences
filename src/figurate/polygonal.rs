use num::BigInt;

use crate::figurate::triangular::Triangular;

/// The polygonal numbers with selectable order.
pub struct Polygonal {
    val: BigInt,
    ctr: BigInt,
    inc: BigInt,
}

impl Polygonal {
    /// k = 0 -> The natural numbers
    /// 0, 1, 2, 3, 4, 5, 6, 7, 8, 9...
    /// k = 1 -> The triangular numbers
    /// 0, 1, 3, 6, 10, 15, 21, 28, 36, 45...
    pub fn new(k: i64) -> Self {
        Self {
            val: BigInt::from(0),
            ctr: BigInt::from(1),
            inc: BigInt::from(k),
        }
    }

    /// The nth polygonal number of order k
    pub fn nth(k: i64, n: u64) -> BigInt {
        let k = &BigInt::from(k);
        let n = &BigInt::from(n);
        ((k - 2) * n * n - (k - 4) * n) / 2
    }
}

impl Iterator for Polygonal {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val += &self.ctr;
        self.ctr += &self.inc;
        Some(out)
    }
}

/// The centered polygonal numbers with selectable order
pub struct CenteredPolygonal {
    k: BigInt,
    triangular: Polygonal,
}

impl CenteredPolygonal {
    /// k = 3 -> Centered triangular numbers
    /// 1, 4, 10, 19, 31, 46, 64, 85, 109, 136...
    /// k = 4 -> Centered square numbers
    /// 1, 5, 13, 25, 41, 61, 85, 113, 145, 181...
    /// Lower values of k are consistent but do not have standard names
    pub fn new(k: i64) -> Self {
        Self {
            k: BigInt::from(k),
            triangular: Polygonal::new(1),
        }
    }

    /// The nth centered polygonal number of order k
    pub fn nth(k: i64, n: u64) -> BigInt {
        (Triangular::nth(n) * k) + 1
    }
}

impl Iterator for CenteredPolygonal {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.triangular.next()? * &self.k + 1;
        Some(out)
    }
}

// These low order values are the same as two versions of Hogben's central polygonal numbers
crate::print_a_few!(
    CenteredPolygonal::new(1), 0, 10;
    CenteredPolygonal::new(2), 0, 10;
);

crate::check_sequences!(
    Polygonal::new(0), 0, 10, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    Polygonal::new(1), 0, 10, [0, 1, 3, 6, 10, 15, 21, 28, 36, 45];
    Polygonal::new(2), 0, 10, [0, 1, 4, 9, 16, 25, 36, 49, 64, 81];
    Polygonal::new(3), 0, 10, [0, 1, 5, 12, 22, 35, 51, 70, 92, 117];
    CenteredPolygonal::new(3), 0, 10, [1, 4, 10, 19, 31, 46, 64, 85, 109, 136];
    CenteredPolygonal::new(4), 0, 10, [1, 5, 13, 25, 41, 61, 85, 113, 145, 181];
    CenteredPolygonal::new(5), 0, 10, [1, 6, 16, 31, 51, 76, 106, 141, 181, 226];
    CenteredPolygonal::new(6), 0, 10, [1, 7, 19, 37, 61, 91, 127, 169, 217, 271];
);
