use crate::core::summation::PartialSums;
use num::{
    BigInt, BigRational, CheckedAdd, CheckedMul, Integer, One, Signed, Zero, bigint::Sign,
    rational::Ratio,
};

/// The terms of the harmonic series.
/// 1, 1/2, 1/3, 1/4, 1/5...
pub struct Harmonic<T> {
    ctr: Ratio<T>,
}

impl<T: CheckedAdd + CheckedMul + Clone + Integer> Harmonic<T> {
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

impl<T: CheckedAdd + CheckedMul + Clone + Integer> Iterator for Harmonic<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr = self.ctr.checked_add(&Ratio::one())?;
        Some(self.ctr.recip())
    }
}

/// The terms of the alternating harmonic series.
/// 1, -1/2, 1/3, -1/4, 1/5...
pub struct HarmonicAlternating<T> {
    ctr: Ratio<T>,
    sign: Sign,
}

impl<T: CheckedAdd + CheckedMul + Clone + Integer + Signed> HarmonicAlternating<T> {
    pub fn new() -> Self {
        Self {
            ctr: Ratio::zero(),
            sign: Sign::Minus,
        }
    }
}

impl HarmonicAlternating<BigInt> {
    pub fn new_big() -> Self {
        Self {
            ctr: BigRational::zero(),
            sign: Sign::Minus,
        }
    }
}

impl<T: CheckedAdd + CheckedMul + Clone + Integer + Signed> Iterator for HarmonicAlternating<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr = self.ctr.checked_add(&Ratio::one())?;
        self.sign = -self.sign;
        match self.sign {
            Sign::Minus => Some(-self.ctr.recip()),
            Sign::NoSign => unreachable!(),
            Sign::Plus => Some(self.ctr.recip()),
        }
    }
}

/// The partial sums of the harmonic series.
/// 0, 1, 3/2, 11/6, 25/12, 137/60, 49/20, 363/140...
pub struct HarmonicNumbers<T> {
    series: PartialSums<Ratio<T>>,
}

impl<T: CheckedAdd + CheckedMul + Clone + Integer + 'static> HarmonicNumbers<T> {
    pub fn new() -> Self {
        Self {
            series: PartialSums::new(Harmonic::new()),
        }
    }
}

impl HarmonicNumbers<BigInt> {
    pub fn new_big() -> Self {
        Self {
            series: PartialSums::new(Harmonic::new()),
        }
    }
}

impl<T: CheckedAdd + CheckedMul + Clone + Integer> Iterator for HarmonicNumbers<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.series.next()
    }
}

/// The partial sums of the harmonic series.
/// 0, 1, 1/2, 5/6, 7/12, 47/60, 37/60...
pub struct AlternatingHarmonicNumbers<T> {
    series: PartialSums<Ratio<T>>,
}

impl<T: CheckedAdd + CheckedMul + Clone + Integer + Signed + 'static>
    AlternatingHarmonicNumbers<T>
{
    pub fn new() -> Self {
        Self {
            series: PartialSums::new(HarmonicAlternating::new()),
        }
    }
}

impl AlternatingHarmonicNumbers<BigInt> {
    pub fn new_big() -> Self {
        Self {
            series: PartialSums::new(HarmonicAlternating::new()),
        }
    }
}

impl<T: CheckedAdd + CheckedMul + Clone + Integer> Iterator for AlternatingHarmonicNumbers<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.series.next()
    }
}

#[cfg(test)]
use crate::core::rational_digits::rational_decimal_string;
crate::print_sequences!(
    AlternatingHarmonicNumbers::new_big().map(|q| rational_decimal_string(q, 5).unwrap()), 50, 10; // Converges on ln(2)
);

crate::check_sequences!(
    Harmonic::<i32>::new(), ["1", "1/2", "1/3", "1/4", "1/5"];
    HarmonicNumbers::<i32>::new(), ["0", "1", "3/2", "11/6", "25/12", "137/60", "49/20"];
    AlternatingHarmonicNumbers::<i32>::new(), ["0", "1", "1/2", "5/6", "7/12", "47/60", "37/60"];
);
