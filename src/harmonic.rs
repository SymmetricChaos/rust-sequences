use num::{BigInt, BigRational, CheckedAdd, CheckedMul, Integer, One, Zero, rational::Ratio};

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
    sum: Ratio<T>,
    ctr: Ratio<T>,
}

impl<T: Integer + Clone> HarmonicSums<T> {
    pub fn new() -> Self {
        Self {
            sum: Ratio::zero(),
            ctr: Ratio::zero(),
        }
    }
}

impl HarmonicSums<BigInt> {
    pub fn new_big() -> Self {
        Self {
            sum: BigRational::zero(),
            ctr: BigRational::zero(),
        }
    }
}

impl<T: Integer + Clone + CheckedAdd + CheckedMul> Iterator for HarmonicSums<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.sum.clone();
        self.ctr = self.ctr.checked_add(&Ratio::one())?;
        self.sum = self.sum.checked_add(&self.ctr.recip())?;
        Some(out)
    }
}

crate::print_values!(
    Harmonic::new_big(), 0, 10;
    HarmonicSums::new_big(), 0, 10;
);
