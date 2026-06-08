use crate::{Number, fibonacci::Fibonacci};
use num::{BigInt, CheckedAdd, Integer, rational::Ratio};

/// Convergents of the golden ratio as calculated from the Fibonacci sequence.
///
/// Ratios converge on 1.6180339887...
pub struct GoldenRatio<T> {
    fib: Fibonacci<T>,
    a: T,
}

impl GoldenRatio<Number> {
    pub fn new() -> Self {
        let mut fib = Fibonacci::new();
        fib.next().unwrap();
        let a = fib.next().unwrap();
        Self { fib, a }
    }
}

#[cfg(feature = "big_int")]
impl GoldenRatio<BigInt> {
    pub fn new_big() -> Self {
        let mut fib = Fibonacci::new_big();
        fib.next().unwrap();
        let a = fib.next().unwrap();
        Self { fib, a }
    }
}

impl<T: Clone + CheckedAdd + Integer> Iterator for GoldenRatio<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let b = self.fib.next()?;
        let out = Ratio::new(b.clone(), self.a.clone());
        self.a = b;
        Some(out)
    }
}

#[cfg(test)]
use crate::core::traits::DigitSequence;
crate::print_sequences!(
    GoldenRatio::new_big(), 10;
    GoldenRatio::new_big().map(|q| q.digits(5).unwrap()), 10;
);
