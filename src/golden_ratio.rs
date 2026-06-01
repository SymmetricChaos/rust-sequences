use crate::{Number, fibonacci::Fibonacci};
use num::{BigInt, rational::Ratio};

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

impl GoldenRatio<BigInt> {
    pub fn new_big() -> Self {
        let mut fib = Fibonacci::new_big();
        fib.next().unwrap();
        let a = fib.next().unwrap();
        Self { fib, a }
    }
}

impl Iterator for GoldenRatio<Number> {
    type Item = Ratio<Number>;

    fn next(&mut self) -> Option<Self::Item> {
        let b = self.fib.next()?;
        let out = Ratio::new(b, self.a);
        self.a = b;
        Some(out)
    }
}

impl Iterator for GoldenRatio<BigInt> {
    type Item = Ratio<BigInt>;

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
