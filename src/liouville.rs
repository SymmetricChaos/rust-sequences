use crate::{Number, core::traits::Increment, factorial::Factorial};
use num::{BigInt, CheckedAdd, CheckedMul, Integer, Zero};

/// Liouville's constant, decimal expansion of the first number proven to be transcendental. Also the characteristic function of the factorials.
///
/// ```text
/// 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0...
/// ```
pub struct Liouville<T> {
    factorial: Factorial<T>,
    next_factorial: T,
    ctr: T,
}

impl Liouville<Number> {
    pub fn new() -> Self {
        let mut factorial = Factorial::new();
        factorial.next();
        let next_factorial = factorial.next().unwrap();
        Self {
            factorial,
            next_factorial,
            ctr: 0,
        }
    }
}

impl Liouville<BigInt> {
    pub fn new_big() -> Self {
        let mut factorial = Factorial::new_big();
        factorial.next();
        let next_factorial = factorial.next().unwrap();
        Self {
            factorial,
            next_factorial,
            ctr: BigInt::zero(),
        }
    }
}

impl<T: Clone + CheckedAdd + CheckedMul + Integer> Iterator for Liouville<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.incr()?;
        if self.ctr == self.next_factorial {
            self.next_factorial = self.factorial.next()?;
            return Some(T::one());
        }
        Some(T::zero())
    }
}

crate::check_sequences!(
    Liouville::new(), [1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
);

crate::sample_sequences!(
    Liouville::new();
);
