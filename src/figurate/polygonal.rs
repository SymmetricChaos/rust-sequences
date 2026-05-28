use num::{BigInt, One, Signed, Zero};

use crate::Number;

/// The polygonal numbers with selectable order, k, with k >= 2.
///
/// ```text
/// k = 2
/// 0, 1, 2, 3, 4, 5, 6, 7, 8, 9...
///
/// k = 3
/// 0, 1, 3, 6, 10, 15, 21, 28, 36...
///
/// k = 4
/// 0, 1, 4, 9, 16, 25, 36, 49...
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

    pub fn nth_big<T>(n: T, k: T) -> BigInt
    where
        BigInt: From<T>,
    {
        let k = &BigInt::from(k);
        let n = &BigInt::from(n);
        ((k - 2) * n * n - (k - 4) * n) / 2
    }
}

impl Iterator for Polygonal<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val += &self.gnomon;
        self.gnomon += &self.order;
        Some(out)
    }
}

impl Iterator for Polygonal<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val += &self.gnomon;
        self.gnomon += &self.order;
        Some(out)
    }

    // Optimized .nth() makes .skip() quicker
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.gnomon += &self.order * n;

        let idx = if self.order.is_zero() {
            &self.gnomon
        } else {
            &((&self.gnomon - 1) / &self.order)
        };

        let k = &self.order;
        self.val = (k * idx * idx - (k - 2) * idx) / 2;

        let out = self.val.clone();

        self.val += &self.gnomon;
        self.gnomon += &self.order;

        Some(out)
    }
}

crate::check_sequences!(
    Polygonal::new(2), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    Polygonal::new(3), [0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 66, 78, 91, 105, 120, 136, 153, 171, 190, 210, 231, 253, 276, 300, 325, 351, 378, 406, 435, 465, 496, 528, 561, 595, 630, 666, 703, 741, 780, 820, 861, 903, 946, 990, 1035, 1081, 1128, 1176, 1225, 1275, 1326, 1378, 1431];
);
