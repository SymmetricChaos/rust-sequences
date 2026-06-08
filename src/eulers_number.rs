use crate::{Number, factorial::Factorial};
use num::{BigInt, CheckedAdd, CheckedMul, Integer, Zero, rational::Ratio};

/// Convergents of e as the series of partial sums of the reciprocals of factorials.
///
/// ```text
/// numerators
/// 0, 1, 2, 5, 8, 65, 163, 1957, 685, 109601, 98641, 9864101, 13563139...
///
/// demoninators
/// 1, 1, 1, 2, 3, 24, 60, 720, 252, 40320, 36288, 3628800, 4989600...
/// ```
pub struct Euler<T> {
    factorials: Factorial<T>,
    sum: Ratio<T>,
}

impl Euler<Number> {
    pub fn new() -> Self {
        Self {
            factorials: Factorial::new(),
            sum: Ratio::zero(),
        }
    }

    pub fn numers() -> impl Iterator<Item = Number> {
        Self::new().map(|q| *q.numer())
    }

    pub fn denoms() -> impl Iterator<Item = Number> {
        Self::new().map(|q| *q.denom())
    }
}

#[cfg(feature = "big_int")]
impl Euler<BigInt> {
    pub fn new_big() -> Self {
        Self {
            factorials: Factorial::new_big(),
            sum: Ratio::zero(),
        }
    }

    pub fn numers_big() -> impl Iterator<Item = BigInt> {
        Self::new_big().map(|q| q.numer().clone())
    }

    pub fn denoms_big() -> impl Iterator<Item = BigInt> {
        Self::new_big().map(|q| q.denom().clone())
    }
}

impl<T: Clone + Integer + CheckedAdd + CheckedMul> Iterator for Euler<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.sum.clone();
        let q = Ratio::new_raw(T::one(), self.factorials.next()?);
        self.sum = match self.sum.checked_add(&q) {
            Some(s) => s,
            None => return Some(out),
        };
        Some(out)
    }
}

crate::check_sequences!(
    Euler::numers_big(), [0_i128, 1, 2, 5, 8, 65, 163, 1957, 685, 109601, 98641, 9864101, 13563139, 260412269, 8463398743, 47395032961, 888656868019, 56874039553217, 7437374403113, 17403456103284421, 82666416490601, 6613313319248080001, 69439789852104840011];
    Euler::denoms_big(), [1_i128, 1, 1, 2, 3, 24, 60, 720, 252, 40320, 36288, 3628800, 4989600, 95800320, 3113510400, 17435658240, 326918592000, 20922789888000, 2736057139200, 6402373705728000, 30411275102208, 2432902008176640000, 25545471085854720000, 224800145555521536000];
);

crate::sample_sequences!(
    Euler::numers_big();
    Euler::denoms_big();
);
