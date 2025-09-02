use num::{BigInt, BigRational};

/// The terms of the harmonic series.
/// 1, 1/2, 1/3, 1/4, 1/5...
pub struct Harmonic {
    ctr: BigRational,
}

impl Harmonic {
    pub fn new() -> Self {
        Self {
            ctr: BigRational::from_integer(BigInt::from(0)),
        }
    }
}

impl Iterator for Harmonic {
    type Item = BigRational;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr += BigRational::from_integer(BigInt::from(1));
        Some(self.ctr.recip())
    }
}

/// The partial sums of the harmonic series.
/// 0, 1, 3/2, 11/6, 25/12, 137/60, 49/20, 363/140...
pub struct HarmonicSums {
    val: BigRational,
    ctr: BigRational,
}

impl HarmonicSums {
    pub fn new() -> Self {
        Self {
            val: BigRational::from_integer(BigInt::from(0)),
            ctr: BigRational::from_integer(BigInt::from(1)),
        }
    }
}

impl Iterator for HarmonicSums {
    type Item = BigRational;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val += self.ctr.recip();
        self.ctr += BigRational::from_integer(BigInt::from(1));
        Some(out)
    }
}

crate::print_a_few!(
    Harmonic::new(), 0, 10;
    HarmonicSums::new(), 0, 8;
);
