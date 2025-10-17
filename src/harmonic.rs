use num::{BigRational, FromPrimitive};

use crate::core::summation::PartialSums;

/// The terms of the harmonic series.
/// 1, 1/2, 1/3, 1/4, 1/5...
pub struct Harmonic {
    ctr: BigRational,
}

impl Harmonic {
    pub fn new_big() -> Self {
        Self {
            ctr: BigRational::from_i32(0).unwrap(),
        }
    }
}

impl Iterator for Harmonic {
    type Item = BigRational;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr += BigRational::from_i32(1).unwrap();
        Some(self.ctr.recip())
    }
}

/// The partial sums of the harmonic series.
/// 0, 1, 3/2, 11/6, 25/12, 137/60, 49/20, 363/140...
pub struct HarmonicSums {
    ctr: PartialSums<BigRational>,
}

impl HarmonicSums {
    pub fn new_big() -> Self {
        Self {
            ctr: PartialSums::new(Harmonic::new_big()),
        }
    }
}

impl Iterator for HarmonicSums {
    type Item = BigRational;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.next()
    }
}

crate::print_values!(
    Harmonic::new_big(), 0, 10;
    HarmonicSums::new_big(), 0, 8;
);
