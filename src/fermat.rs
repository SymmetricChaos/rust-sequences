use crate::Number;
use num::{BigInt, CheckedAdd, CheckedMul, Integer};

/// The Fermat numbers. (2^(2^n))+1 for positive integers n. Terms grow extremely quickly.
///
/// ```text
/// 3, 5, 17, 257, 65537, 4294967297, 18446744073709551617...
/// ```
pub struct Fermat<T> {
    prev: T,
    overflowed: bool,
}

impl Fermat<Number> {
    pub fn new() -> Self {
        Self {
            prev: 3,
            overflowed: false,
        }
    }
}

#[cfg(feature = "big_int")]
impl Fermat<BigInt> {
    pub fn new_big() -> Self {
        Self {
            prev: BigInt::from(3),
            overflowed: false,
        }
    }
}

impl<T: Clone + CheckedAdd + CheckedMul + Integer> Iterator for Fermat<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.overflowed {
            return None;
        }
        let out = self.prev.clone();

        let t = self.prev.clone() - T::one();
        match t.checked_mul(&t) {
            Some(x) => match x.checked_add(&T::one()) {
                Some(n) => self.prev = n,
                None => return Some(out),
            },
            None => return Some(out),
        }

        Some(out)
    }
}

crate::check_sequences!(
    Fermat::new(), [3_u64, 5, 17, 257, 65537, 4294967297];
    Fermat::new_big(), [3_u64, 5, 17, 257, 65537, 4294967297];
);

crate::sample_sequences!(
    Fermat::new_big();
);
