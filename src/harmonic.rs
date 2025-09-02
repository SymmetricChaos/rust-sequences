use num::{BigInt, BigRational};

/// The terms of the harmonic series.
/// 1, 1/2, 1/3, 1/4, 1/5...
pub struct Harmonic {
    ctr: BigRational,
}

impl Harmonic {
    pub fn new() -> Self {
        Self {
            ctr: BigRational::from_integer(BigInt::from(1)),
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

crate::print_a_few!(
    Harmonic::new(), 0, 10;
);
