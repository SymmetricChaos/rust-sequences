use num::BigInt;

use crate::core::integer::Integers;

/// The generalized polygonal numbers with selectable order. Extends the domain of the polygonal numbers to all integers.
///
/// ```text
/// k = -2
/// 0, 1, -5, -2, -14, -9, -27, -20, -44, -35...
///
/// k = -1
/// 0, 1, -4, -1, -11, -6, -21, -14, -34, -25...
///
/// k = 0
/// 0, 1, -3, 0, -8, -3, -15, -8, -24, -15...
///
/// k = 1
/// 0, 1, -2, 1, -5, 0, -9, -2, -14, -5...
///
/// k = 2
/// 0, 1, -1, 2, -2, 3, -3, 4, -4, 5...
/// ```
pub struct GeneralizedPolygonal {
    integers: Integers<BigInt>,
    k: BigInt,
}

impl GeneralizedPolygonal {
    pub fn new_big<T>(k: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            integers: Integers::new_big(),
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
}

crate::check_iteration_times!(
    GeneralizedPolygonal::new_big(5), 700_000;
);

crate::print_sequences!(
    GeneralizedPolygonal::new_big(-2), 10;
    GeneralizedPolygonal::new_big(-1), 10;
    GeneralizedPolygonal::new_big(0), 10;
    GeneralizedPolygonal::new_big(1), 10;
    GeneralizedPolygonal::new_big(2), 10;
    GeneralizedPolygonal::new_big(3), 10;
);

crate::check_sequences!(
    GeneralizedPolygonal::new_big(5), [0, 1, 2, 5, 7, 12, 15, 22, 26, 35, 40, 51, 57, 70, 77, 92, 100, 117, 126, 145, 155, 176, 187, 210, 222, 247, 260, 287, 301, 330, 345, 376, 392, 425, 442, 477, 495, 532, 551, 590, 610, 651, 672, 715, 737, 782, 805, 852, 876, 925, 950, 1001, 1027, 1080, 1107, 1162, 1190, 1247, 1276, 1335]; // Generalized pentagonal numbers are particularly important
);
