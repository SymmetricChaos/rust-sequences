use crate::{Number, core::traits::Increment};
use num::{BigInt, CheckedAdd, One, Zero, rational::Ratio};

/// The partial sums of Kempner's series. Similar to the harmonic series of terms where the denominator contains a 9 in the decimal expansion are exluded. The sum converges (very slowly) on a value of about 22.92.
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

impl Iterator for Kempner<Number> {
    type Item = Ratio<Number>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut n = self.ctr;
            let mut pos = 1;
            loop {
                // doesn't contain 9 so include it
                if n == 0 {
                    self.sum = self.sum.checked_add(&Ratio::new(1, self.ctr))?;
                    self.ctr.incr()?;
                    return Some(self.sum.clone());
                }
                // contains a 9 so skip forward to the next numbers without one
                if n % self.base == self.digit {
                    self.ctr = self.ctr.checked_add(pos)?;
                    break;
                // keep searching for a 9
                } else {
                    n /= self.base;
                    pos *= self.base;
                }
            }
        }
    }
}

impl Iterator for Kempner<BigInt> {
    type Item = Ratio<BigInt>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut n = self.ctr.clone();
            let mut pos = BigInt::one();
            loop {
                // doesn't contain 9 so include it
                if n.is_zero() {
                    self.sum = self
                        .sum
                        .checked_add(&Ratio::new(BigInt::one(), self.ctr.clone()))?;
                    self.ctr.incr()?;
                    return Some(self.sum.clone());
                }
                // contains a 9 so skip forward to the next numbers without one
                if &(&n % &self.base) == &self.digit {
                    self.ctr = self.ctr.checked_add(&pos)?;
                    break;
                // keep searching for a 9
                } else {
                    n = &n / &self.base;
                    pos = &pos * &self.base;
                }
            }
        }
    }
}

#[cfg(test)]
use crate::core::traits::DigitSequence;
crate::print_sequences!(
    Kempner::new_big().map(|q| q.digits(10).unwrap()), skip 1000, 5; // Convergence is very slow but approaches 22.92
);

crate::check_sequences!(
    Kempner::numers(), [1_u64, 3, 11, 25, 137, 49, 363, 761, 789, 8959, 27647, 368651, 377231, 128413, 261831, 4531207, 41461543, 8414831, 8531519, 8642903, 201237217, 203585563, 5145999379, 5200191979, 15757132337, 15908097437, 16048998197, 501745966907];
    Kempner::denoms(), [1_u64, 2, 6, 12, 60, 20, 140, 280, 280, 3080, 9240, 120120, 120120, 40040, 80080, 1361360, 12252240, 2450448, 2450448, 2450448, 56360304, 56360304, 1409007600, 1409007600, 4227022800, 4227022800, 4227022800, 131037706800, 262075413600];
);
