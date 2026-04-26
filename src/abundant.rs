use crate::utils::divisibility::{aliquot_sum, factors, proper_factors};

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
            let factors = proper_factors(self.n);
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

crate::check_sequences!(
    Abundant::new(),          [12, 18, 20, 24, 30, 36, 40, 42, 48, 54, 56, 60, 66, 70, 72, 78, 80, 84, 88, 90, 96, 100, 102, 104, 108, 112, 114, 120, 126, 132, 138, 140, 144, 150, 156, 160, 162, 168, 174, 176, 180, 186, 192, 196, 198, 200, 204, 208, 210, 216, 220, 222, 224, 228, 234, 240, 246, 252, 258, 260, 264, 270];
    PrimitiveAbundant::new(), [12, 18, 20, 30, 42, 56, 66, 70, 78, 88, 102, 104, 114, 138, 174, 186, 196, 222, 246, 258, 272, 282, 304, 308, 318, 354, 364, 366, 368, 402, 426, 438, 464, 474, 476, 498, 532, 534, 550, 572, 582, 606, 618, 642, 644, 650, 654, 678, 748, 762, 786, 812, 822];
);
