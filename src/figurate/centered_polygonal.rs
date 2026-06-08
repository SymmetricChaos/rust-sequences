use crate::{Number, figurate::polygonal::Polygonal};
use num::{BigInt, CheckedAdd, CheckedMul, Integer, One, Signed};

/// The centered polygonal numbers with selectable order, k, k >= 0. There are not standard names for k=1 and k=2 but they are related to Hogben's central polygonal numbers.
///
/// ```text
/// k = 0
/// 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1...
///
/// k = 1
/// 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192...
///
/// k = 2
/// 1, 3, 7, 15, 31, 63, 127, 255, 511, 1023, 2047, 4095, 8191, 16383...
///
/// k = 3 (centered triangular)
/// 1, 4, 10, 22, 46, 94, 190, 382, 766, 1534, 3070, 6142, 12286, 24574...
///
/// k = 4 (centered square)
/// 1, 5, 13, 29, 61, 125, 253, 509, 1021, 2045, 4093, 8189, 16381...
/// ```
pub struct CenteredPolygonal<T> {
    k: T,
    polygonal: Polygonal<T>,
}

impl CenteredPolygonal<Number> {
    pub fn new(k: Number) -> Self {
        assert!(!k.is_negative());
        Self {
            k,
            polygonal: Polygonal::new(3),
        }
    }
}

#[cfg(feature = "big_int")]
impl CenteredPolygonal<BigInt> {
    pub fn new_big<T: One>(k: T) -> Self
    where
        BigInt: From<T>,
    {
        let k = BigInt::from(k);
        assert!(!k.is_negative());
        Self {
            k,
            polygonal: Polygonal::new_big::<Number>(3), // have to specify the type being accepted by Polygonal::new_big
        }
    }
}

impl<T: Clone + CheckedAdd + CheckedMul + Integer> Iterator for CenteredPolygonal<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self
            .polygonal
            .next()?
            .checked_mul(&self.k)?
            .checked_add(&T::one())?;
        Some(out)
    }
}

crate::check_sequences!(
    CenteredPolygonal::new_big(3), [1, 4, 10, 19, 31, 46, 64, 85, 109, 136, 166, 199, 235, 274, 316, 361, 409, 460, 514, 571, 631, 694, 760, 829, 901, 976, 1054, 1135, 1219, 1306, 1396, 1489, 1585, 1684, 1786, 1891, 1999, 2110, 2224, 2341, 2461, 2584, 2710, 2839, 2971, 3106, 3244, 3385, 3529];
    CenteredPolygonal::new_big(4), [1, 5, 13, 25, 41, 61, 85, 113, 145, 181, 221, 265, 313, 365, 421, 481, 545, 613, 685, 761, 841, 925, 1013, 1105, 1201, 1301, 1405, 1513, 1625, 1741, 1861, 1985, 2113, 2245, 2381, 2521, 2665, 2813, 2965, 3121, 3281, 3445, 3613, 3785, 3961, 4141, 4325, 4513];
    CenteredPolygonal::new_big(5), [1, 6, 16, 31, 51, 76, 106, 141, 181, 226, 276, 331, 391, 456, 526, 601, 681, 766, 856, 951, 1051, 1156, 1266, 1381, 1501, 1626, 1756, 1891, 2031, 2176, 2326, 2481, 2641, 2806, 2976, 3151, 3331, 3516, 3706, 3901, 4101, 4306, 4516, 4731, 4951, 5176, 5406];
    CenteredPolygonal::new_big(6), [1, 7, 19, 37, 61, 91, 127, 169, 217, 271, 331, 397, 469, 547, 631, 721, 817, 919, 1027, 1141, 1261, 1387, 1519, 1657, 1801, 1951, 2107, 2269, 2437, 2611, 2791, 2977, 3169, 3367, 3571, 3781, 3997, 4219, 4447, 4681, 4921, 5167, 5419, 5677, 5941, 6211, 6487, 6769];
);

crate::sample_sequences!(
    CenteredPolygonal::new(0);
    CenteredPolygonal::new(1);
    CenteredPolygonal::new(2);
    CenteredPolygonal::new(3);
    CenteredPolygonal::new(4);
);
