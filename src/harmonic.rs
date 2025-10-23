use num::{BigInt, BigRational, CheckedAdd, CheckedMul, Integer, One, Zero, rational::Ratio};

use crate::core::summation::PartialSums;

/// The terms of the harmonic series.
/// 1, 1/2, 1/3, 1/4, 1/5...
pub struct Harmonic<T> {
    ctr: Ratio<T>,
}

impl<T: Integer + Clone> Harmonic<T> {
    pub fn new() -> Self {
        Self { ctr: Ratio::zero() }
    }
}

impl Harmonic<BigInt> {
    pub fn new_big() -> Self {
        Self {
            ctr: BigRational::zero(),
        }
    }
}

impl<T: Integer + Clone + CheckedAdd + CheckedMul> Iterator for Harmonic<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr = self.ctr.checked_add(&Ratio::one())?;
        Some(self.ctr.recip())
    }
}

/// The partial sums of the harmonic series.
/// 0, 1, 3/2, 11/6, 25/12, 137/60, 49/20, 363/140...
pub struct HarmonicSums<T> {
    ctr: PartialSums<Ratio<T>>,
}

impl<T: Integer + Clone + CheckedAdd + CheckedMul + 'static> HarmonicSums<T> {
    pub fn new() -> Self {
        Self {
            ctr: PartialSums::new(Harmonic::new()),
        }
    }
}

impl HarmonicSums<BigInt> {
    pub fn new_big() -> Self {
        Self {
            ctr: PartialSums::new(Harmonic::new_big()),
        }
    }
}

impl<T: Integer + Clone + CheckedAdd + CheckedMul> Iterator for HarmonicSums<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.next()
    }
}

crate::print_values!(
    Harmonic::new_big(), 0, 10;
    HarmonicSums::new_big(), 0, 8;
);
