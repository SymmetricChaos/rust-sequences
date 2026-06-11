use crate::{Number, core::traits::Increment};
use num::{BigInt, CheckedAdd, CheckedDiv, CheckedMul, Integer, One, Zero, rational::Ratio};

/// The partial sums of Kempner's series. Similar to the harmonic series of terms where the denominator contains a 9 in the decimal expansion are exluded. The sum converges (very slowly) on a value of about 22.92.
///
/// ```text
/// 1, 3/2, 11/6, 25/12, 137/60, 49/20, 363/140, 761/280, 789/280...
/// ```
pub struct Kempner<T> {
    ctr: T,
    sum: Ratio<T>,
    digit: T,
    base: T,
}

impl Kempner<Number> {
    pub fn new() -> Self {
        Self {
            ctr: 1,
            sum: Ratio::new(0, 1),
            digit: 9,
            base: 10,
        }
    }

    pub fn numers() -> impl Iterator<Item = Number> {
        Self::new().map(|q| q.numer().clone())
    }

    pub fn denoms() -> impl Iterator<Item = Number> {
        Self::new().map(|q| q.denom().clone())
    }
}

#[cfg(feature = "big_int")]
impl Kempner<BigInt> {
    pub fn new_big() -> Self {
        Self {
            ctr: BigInt::one(),
            sum: Ratio::new(BigInt::zero(), BigInt::one()),
            digit: BigInt::from(9),
            base: BigInt::from(10),
        }
    }

    pub fn numers_big() -> impl Iterator<Item = BigInt> {
        Self::new_big().map(|q| q.numer().clone())
    }

    pub fn denoms_big() -> impl Iterator<Item = BigInt> {
        Self::new_big().map(|q| q.denom().clone())
    }
}

impl<T: Clone + CheckedAdd + CheckedDiv + CheckedMul + Integer> Iterator for Kempner<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut n = self.ctr.clone();
            let mut pos = T::one();
            loop {
                // doesn't contain 9 so include it
                if n.is_zero() {
                    self.sum = self
                        .sum
                        .checked_add(&Ratio::new(T::one(), self.ctr.clone()))?;
                    self.ctr.incr()?;
                    return Some(self.sum.clone());
                }
                // contains a 9 so skip forward to the next numbers without one
                if n.clone() % self.base.clone() == self.digit {
                    self.ctr = self.ctr.checked_add(&pos)?;
                    break;
                // keep searching for a 9
                } else {
                    n = n.checked_div(&self.base)?;
                    pos = pos.checked_mul(&self.base)?;
                }
            }
        }
    }
}

crate::check_sequences!(
    Kempner::numers(), [1_u64, 3, 11, 25, 137, 49, 363, 761, 789, 8959, 27647, 368651, 377231, 128413, 261831, 4531207, 41461543, 8414831, 8531519, 8642903, 201237217, 203585563, 5145999379, 5200191979, 15757132337, 15908097437, 16048998197, 501745966907];
    Kempner::denoms(), [1_u64, 2, 6, 12, 60, 20, 140, 280, 280, 3080, 9240, 120120, 120120, 40040, 80080, 1361360, 12252240, 2450448, 2450448, 2450448, 56360304, 56360304, 1409007600, 1409007600, 4227022800, 4227022800, 4227022800, 131037706800, 262075413600];
);

crate::sample_sequences!(
    Kempner::new();
);
