use num::rational::Ratio;

use crate::utils::divisibility::{
    abundancy_index, aliquot_sum, proper_divisors, radical, sum_of_divisors,
};

/// The abundant numbers, those which have an aliquot sum greater than themselves.
///
/// 12, 18, 20, 24, 30, 36, 40, 42, 48, 54...
pub struct Abundant {
    n: u64,
}

impl Abundant {
    /// Only u64 output supported.
    pub fn new() -> Self {
        Self { n: 11 }
    }
}

impl Iterator for Abundant {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.n = self.n.checked_add(1)?;
            if aliquot_sum(self.n).unwrap() > self.n {
                return Some(self.n);
            }
        }
    }
}

/// The primitive abundant numbers. Abundant numbers that have no abundant factors.
///
/// 12, 18, 20, 30, 42, 56, 66, 70, 78, 88...
pub struct PrimitiveAbundant {
    n: u64,
    terms: Vec<u64>,
}

impl PrimitiveAbundant {
    /// Only u64 output supported.
    pub fn new() -> Self {
        Self {
            n: 11,
            terms: Vec::new(),
        }
    }
}

impl Iterator for PrimitiveAbundant {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        'outer: loop {
            self.n = self.n.checked_add(1)?;
            let factors = proper_divisors(self.n);
            if aliquot_sum(self.n).unwrap() > self.n {
                self.terms.push(self.n);
                for factor in factors.iter() {
                    if self.terms.contains(factor) {
                        continue 'outer;
                    }
                }
                return Some(self.n);
            }
        }
    }
}

/// Highly abundant numbers. Positive integers for which the sum of divisors is greater than for any small number.
///
/// 1, 2, 3, 4, 6, 8, 10, 12, 16, 18, 20, 24...
pub struct HighlyAbundant {
    record: u64,
    n: u64,
}

impl HighlyAbundant {
    /// Only u64 output supported.
    pub fn new() -> Self {
        Self { record: 0, n: 0 }
    }
}

impl Iterator for HighlyAbundant {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.n = self.n.checked_add(1)?;
            let s = sum_of_divisors(self.n)?;
            if s > self.record {
                self.record = s;
                return Some(self.n);
            }
        }
    }
}

/// Superabundant numbers. Positive integers for which the ratio between the sum of divisors and itself is greater than for any small number.
///
/// 1, 2, 4, 6, 12, 24, 36, 48, 60, 120...
pub struct Superabundant {
    record: Ratio<u64>,
    n: u64,
    step: u64,
}

impl Superabundant {
    /// Only u64 output supported.
    pub fn new() -> Self {
        Self {
            record: Ratio::ZERO,
            n: 0,
            step: 1,
        }
    }
}

impl Iterator for Superabundant {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.n = self.n.checked_add(self.step)?;
            let ab_idx = abundancy_index(self.n)?;
            if ab_idx > self.record {
                self.step = radical(self.n); // Each superabundant number has a primorial factor that is greater than to equal to the greatest primorial factor of the previous
                self.record = ab_idx;
                return Some(self.n);
            }
        }
    }
}

crate::check_sequences!(
    Abundant::new(),          [12, 18, 20, 24, 30, 36, 40, 42, 48, 54, 56, 60, 66, 70, 72, 78, 80, 84, 88, 90, 96, 100, 102, 104, 108, 112, 114, 120, 126, 132, 138, 140, 144, 150, 156, 160, 162, 168, 174, 176, 180, 186, 192, 196, 198, 200, 204, 208, 210, 216, 220, 222, 224, 228, 234, 240, 246, 252, 258, 260, 264, 270];
    PrimitiveAbundant::new(), [12, 18, 20, 30, 42, 56, 66, 70, 78, 88, 102, 104, 114, 138, 174, 186, 196, 222, 246, 258, 272, 282, 304, 308, 318, 354, 364, 366, 368, 402, 426, 438, 464, 474, 476, 498, 532, 534, 550, 572, 582, 606, 618, 642, 644, 650, 654, 678, 748, 762, 786, 812, 822];
    Superabundant::new(),     [1, 2, 4, 6, 12, 24, 36, 48, 60, 120, 180, 240, 360, 720, 840, 1260, 1680, 2520, 5040, 10080, 15120, 25200, 27720, 55440, 110880, 166320, 277200, 332640, 554400, 665280, 720720, 1441440, 2162160, 3603600, 4324320, 7207200, 8648640, 10810800, 21621600, 36756720, 61261200, 73513440, 122522400];
    HighlyAbundant::new(),    [1, 2, 3, 4, 6, 8, 10, 12, 16, 18, 20, 24, 30, 36, 42, 48, 60, 72, 84, 90, 96, 108, 120, 144, 168, 180, 210, 216, 240, 288, 300, 336, 360, 420, 480, 504, 540, 600, 630, 660, 720, 840, 960, 1008, 1080, 1200, 1260, 1440, 1560, 1620, 1680, 1800, 1920, 1980, 2100];
);
