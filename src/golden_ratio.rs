use crate::{Number, fibonacci::Fibonacci};
use num::{BigInt, CheckedAdd, Integer, rational::Ratio};

/// Convergents of the golden ratio as calculated from the Fibonacci sequence.
///
/// ```text
/// 1, 2, 3/2, 5/3, 8/5, 13/8, 21/13, 34/21, 55/34, 89/55, 144/89...
/// ```
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

crate::sample_sequences!(
    GoldenRatio::new_big();
);
