use crate::{Number, fibonacci::Fibonacci};
use num::{BigInt, CheckedAdd, CheckedMul, Integer};

/// The golden rectangle numbers. The area of triangles that converge on having sides with lengths that are the golden ration.
///
/// ```text
/// 0, 1, 2, 6, 15, 40, 104, 273, 714, 1870, 4895, 12816, 33552, 87841...
/// ```
pub struct GoldenRectangle<T> {
    fib: Fibonacci<T>,
    a: T,
}

impl GoldenRectangle<Number> {
    pub fn new() -> Self {
        let mut fib = Fibonacci::new();
        let a = fib.next().unwrap();
        Self { fib, a }
    }
}

impl GoldenRectangle<BigInt> {
    pub fn new_big() -> Self {
        let mut fib = Fibonacci::new_big();
        let a = fib.next().unwrap();
        Self { fib, a }
    }
}

impl<T: Clone + CheckedAdd + CheckedMul + Integer> Iterator for GoldenRectangle<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let b = self.fib.next()?;
        let out = self.a.checked_mul(&b)?;
        self.a = b.clone();
        Some(out)
    }
}

/// The sides lengths of the golden rectangle numbers. A window of two terms of the Fibonacci sequences.
///
/// ```text
/// (0, 1), (1, 1), (1, 2), (2, 3), (3, 5), (5, 8), (8, 13), (13, 21)...
/// ```
pub struct GoldenRectangleSides<T> {
    fib: Fibonacci<T>,
    a: T,
}

impl GoldenRectangleSides<Number> {
    pub fn new() -> Self {
        let mut fib = Fibonacci::new();
        let a = fib.next().unwrap();
        Self { fib, a }
    }
}

impl GoldenRectangleSides<BigInt> {
    pub fn new_big() -> Self {
        let mut fib = Fibonacci::new_big();
        let a = fib.next().unwrap();
        Self { fib, a }
    }
}

impl<T: Clone + CheckedAdd + Integer> Iterator for GoldenRectangleSides<T> {
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let b = self.fib.next()?;
        let out = (self.a.clone(), b.clone());
        self.a = b;
        Some(out)
    }
}

crate::check_sequences!(
    GoldenRectangle::new_big(), [0_u64, 1, 2, 6, 15, 40, 104, 273, 714, 1870, 4895, 12816, 33552, 87841, 229970, 602070, 1576239, 4126648, 10803704, 28284465, 74049690, 193864606, 507544127, 1328767776, 3478759200, 9107509825, 23843770274, 62423800998, 163427632719, 427859097160, 1120149658760];
    GoldenRectangleSides::new_big().flat_map(|f| [f.0,f.1]), [0, 1, 1, 1, 1, 2, 2, 3, 3, 5, 5, 8, 8, 13, 13, 21, 21, 34, 34, 55, 55, 89, 89, 144, 144, 233, 233, 377, 377, 610, 610, 987, 987, 1597, 1597, 2584, 2584, 4181, 4181, 6765, 6765, 10946, 10946, 17711, 17711, 28657, 28657, 46368, 46368, 75025, 75025, 121393];
);

crate::sample_sequences!(
    GoldenRectangle::new();
    GoldenRectangleSides::new().map(|x| format!("{:?}",x));
);
