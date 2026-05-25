use crate::factorial::Factorials;
use num::{BigInt, One, Zero};
use std::marker::PhantomData;

/// Liouville's constant, decimal expansion of the first number proven to be transcendental. Also the characteristic function of the factorials.
///
/// ```text
/// 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0...
/// ```
pub struct Liouville<T> {
    factorial: Factorials<BigInt>,
    next_factorial: BigInt,
    ctr: BigInt,
    _phantom: PhantomData<T>,
}

impl<T: One + Zero> Liouville<T> {
    pub fn new() -> Self {
        let mut factorial = Factorials::new();
        factorial.next();
        let next_factorial = factorial.next().unwrap();
        Self {
            factorial,
            next_factorial,
            ctr: BigInt::zero(),
            _phantom: PhantomData,
        }
    }
}

impl<T: One + Zero> Iterator for Liouville<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr += 1;
        if self.ctr == self.next_factorial {
            self.next_factorial = self.factorial.next()?;
            return Some(T::one());
        }
        Some(T::zero())
    }
}

crate::check_sequences!(
    Liouville::<u64>::new(), [1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
);
