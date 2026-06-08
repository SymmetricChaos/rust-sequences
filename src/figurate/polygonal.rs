use num::{BigInt, CheckedAdd, Integer, One, Signed, Zero};

use crate::Number;

/// The polygonal numbers with selectable order, k, with k >= 2.
///
/// ```text
/// k = 2 (natural numbers)
/// 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18...
///
/// k = 3 (triangular numbers)
/// 0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 66, 78, 91, 105, 120, 136...
///
/// k = 4 (square numbers)
/// 0, 1, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144, 169, 196, 225...
/// ```
pub struct Polygonal<T> {
    val: T,
    gnomon: T,
    order: T,
}

impl Polygonal<Number> {
    pub fn new(k: Number) -> Self {
        let order = k - 2;
        assert!(!order.is_negative());
        Self {
            val: 0,
            gnomon: 1,
            order,
        }
    }
}

#[cfg(feature = "big_int")]
impl Polygonal<BigInt> {
    pub fn new_big<T>(k: T) -> Self
    where
        BigInt: From<T>,
    {
        let order: BigInt = BigInt::from(k) - 2;
        assert!(!order.is_negative());
        Self {
            val: BigInt::zero(),
            gnomon: BigInt::one(),
            order,
        }
    }
}

impl<T: Clone + CheckedAdd + Integer> Iterator for Polygonal<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.val.checked_add(&self.gnomon)?;
        self.gnomon = self.gnomon.checked_add(&self.order)?;
        Some(out)
    }
}

crate::check_sequences!(
    Polygonal::new(2), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    Polygonal::new(3), [0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 66, 78, 91, 105, 120, 136, 153, 171, 190, 210, 231, 253, 276, 300, 325, 351, 378, 406, 435, 465, 496, 528, 561, 595, 630, 666, 703, 741, 780, 820, 861, 903, 946, 990, 1035, 1081, 1128, 1176, 1225, 1275, 1326, 1378, 1431];
);

crate::sample_sequences!(
    Polygonal::new(2);
    Polygonal::new(3);
    Polygonal::new(4);
);
