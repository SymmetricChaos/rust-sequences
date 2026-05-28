use crate::core::traits::Increment;
use num::{CheckedAdd, rational::Ratio};

fn contains_nine(mut n: u64) -> bool {
    while n != 0 {
        if n % 10 == 9 {
            return true;
        } else {
            n = n / 10;
        }
    }
    false
}

/// The partial sums of Kempner's series. Similar to the harmonic series of terms where the denominator contains a 9 in the decimal expansion are exluded. The sum converges (very slowly) on a value of about 22.92.
pub struct Kempner {
    ctr: u64,
    sum: Ratio<u64>,
}

impl Kempner {
    pub fn new() -> Self {
        Self {
            ctr: 0,
            sum: Ratio::new(0, 1),
        }
    }

    pub fn numers() -> impl Iterator<Item = u64> {
        Self::new().map(|q| q.numer().clone())
    }
    pub fn denoms() -> impl Iterator<Item = u64> {
        Self::new().map(|q| q.denom().clone())
    }
}

impl Iterator for Kempner {
    type Item = Ratio<u64>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.ctr.incr();
            if !contains_nine(self.ctr) {
                self.sum = self.sum.checked_add(&Ratio::new(1, self.ctr))?;
                return Some(self.sum);
            }
        }
    }
}

#[cfg(test)]
use crate::core::traits::DigitSequence;
crate::print_sequences!(
    Kempner::new().map(|q| q.digits(10).unwrap()), skip 30, 5;
);

crate::check_sequences!(
    Kempner::numers(), [1_u64, 3, 11, 25, 137, 49, 363, 761, 789, 8959, 27647, 368651, 377231, 128413, 261831, 4531207, 41461543, 8414831, 8531519, 8642903, 201237217, 203585563, 5145999379, 5200191979, 15757132337, 15908097437, 16048998197, 501745966907];
    Kempner::denoms(), [1_u64, 2, 6, 12, 60, 20, 140, 280, 280, 3080, 9240, 120120, 120120, 40040, 80080, 1361360, 12252240, 2450448, 2450448, 2450448, 56360304, 56360304, 1409007600, 1409007600, 4227022800, 4227022800, 4227022800, 131037706800, 262075413600];
);
