use num::rational::Ratio;

use crate::utils::divisibility::aliquot_sum;

/// The abundance of each positive integer. Its aliquot sum minus itself.
///
/// -1, -1, -2, -1, -4, 0, -6, -1, -5, -2, -10, 4, -12, -4, -6...
pub struct Abundance {
    n: i64,
}

impl Abundance {
    /// Only i64 outupt is supported.
    pub fn new() -> Self {
        Self { n: 0 }
    }
}

impl Iterator for Abundance {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.n = self.n.checked_add(1)?;
        let x: i64 = aliquot_sum(self.n.try_into().unwrap())?
            .try_into()
            .expect("fail to convert");
        Some(x - self.n)
    }
}

/// The abundancy index of each positive integer. The sum of its divisors divided by itself.
///
/// 1/1, 3/2, 4/3, 7/4, 6/5, 2/1, 8/7, 15/8
pub struct AbundancyIndex {
    n: u64,
}

impl AbundancyIndex {
    /// Only Ratio<u64> outupt is supported.
    pub fn new() -> Self {
        Self { n: 0 }
    }

    /// Only u64 outupt is supported.
    pub fn new_numers() -> impl Iterator {
        Self::new().map(|x| *x.numer())
    }

    /// Only u64 outupt is supported.
    pub fn new_denoms() -> impl Iterator {
        Self::new().map(|x| *x.denom())
    }
}

impl Iterator for AbundancyIndex {
    type Item = Ratio<u64>;

    fn next(&mut self) -> Option<Self::Item> {
        self.n = self.n.checked_add(1)?;
        let x = crate::utils::divisibility::sum_of_divisors(self.n)?;
        Some(Ratio::new(x, self.n))
    }
}

crate::check_sequences!(
    Abundance::new(), [-1, -1, -2, -1, -4, 0, -6, -1, -5, -2, -10, 4, -12, -4, -6];
    AbundancyIndex::new().map(|x| *x.numer()), [1, 3, 4, 7, 6, 2, 8, 15, 13, 9, 12, 7, 14, 12, 8, 31, 18, 13, 20, 21, 32, 18, 24, 5, 31, 21, 40, 2, 30, 12, 32, 63, 16, 27, 48, 91, 38, 30, 56, 9, 42, 16, 44, 21, 26, 36, 48, 31, 57, 93, 24, 49, 54, 20, 72, 15, 80, 45, 60, 14, 62, 48, 104, 127, 84, 24, 68, 63, 32, 72, 72, 65, 74, 57];
    AbundancyIndex::new().map(|x| *x.denom()), [1, 2, 3, 4, 5, 1, 7, 8, 9, 5, 11, 3, 13, 7, 5, 16, 17, 6, 19, 10, 21, 11, 23, 2, 25, 13, 27, 1, 29, 5, 31, 32, 11, 17, 35, 36, 37, 19, 39, 4, 41, 7, 43, 11, 15, 23, 47, 12, 49, 50, 17, 26, 53, 9, 55, 7, 57, 29, 59, 5, 61, 31, 63, 64, 65, 11, 67, 34, 23, 35, 71, 24, 73, 37, 75, 19];
);
