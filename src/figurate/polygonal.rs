use num::{BigInt, One, Signed, Zero};

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
    pub fn new<T>(k: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            val: BigInt::zero(),
            ctr: BigInt::one(),
            inc: BigInt::from(k),
        }
    }

    /// The nth polygonal number of order k
    /// Panics if n or k is negative.
    pub fn nth<T>(k: T, n: T) -> BigInt
    where
        BigInt: From<T>,
    {
        let k = &BigInt::from(k);
        let n = &BigInt::from(n);
        assert!(!n.is_negative());
        assert!(!k.is_negative());
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

// /// The generalized polygonal numbers with selectable order.
// pub struct PolygonalGeneralized {
//     val: BigInt,
//     ctr: BigInt,
//     inc: BigInt,
// }

// impl PolygonalGeneralized {
//     pub fn new<T>(k: T) -> Self
//     where
//         BigInt: From<T>,
//     {
//         Self {
//             val: BigInt::zero(),
//             ctr: BigInt::one(),
//             inc: BigInt::from(k),
//         }
//     }

//     /// The nth polygonal number of order k
//     /// Panics if n or k is negative.
//     pub fn nth<T>(k: T, n: T) -> BigInt
//     where
//         BigInt: From<T>,
//     {
//         let k = &BigInt::from(k);
//         let n = &BigInt::from(n);
//         assert!(!n.is_negative());
//         assert!(!k.is_negative());
//         ((k - 2) * n * n - (k - 4) * n) / 2
//     }
// }

// impl Iterator for PolygonalGeneralized {
//     type Item = BigInt;

//     fn next(&mut self) -> Option<Self::Item> {
//         let out = self.val.clone();
//         self.val += &self.ctr;
//         self.ctr += &self.inc;
//         Some(out)
//     }
// }

crate::check_sequences!(
    Polygonal::new(0), 0, 10, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    Polygonal::new(1), 0, 10, [0, 1, 3, 6, 10, 15, 21, 28, 36, 45];
    Polygonal::new(2), 0, 10, [0, 1, 4, 9, 16, 25, 36, 49, 64, 81];
    Polygonal::new(3), 0, 10, [0, 1, 5, 12, 22, 35, 51, 70, 92, 117];

);

#[test]
fn big() {
    println!("{}", Polygonal::nth(-3, i128::MAX))
}
