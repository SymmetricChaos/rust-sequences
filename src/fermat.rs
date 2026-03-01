use num::{BigInt, CheckedAdd, CheckedMul, Integer};

/// The Fermat numbers. (2^(2^n))+1 for positive integers n. Terms grow extremely quickly u64 can only produce 6 terms and u128 can only produce 7.
/// 3, 5, 17, 257, 65537, 4294967297...
pub struct Fermat<T> {
    prev: T,
    overflowed: bool,
}

impl<T: Integer + Clone> Fermat<T> {
    pub fn new() -> Self {
        Self {
            prev: T::one() + T::one() + T::one(),
            overflowed: false,
        }
    }
}

impl Fermat<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Integer + Clone + CheckedMul + CheckedAdd> Iterator for Fermat<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.overflowed {
            return None;
        }
        let out = self.prev.clone();

        let t = self.prev.clone() - T::one();
        match (t).checked_mul(&t) {
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
    Fermat::<u64>::new(), [3_u64, 5, 17, 257, 65537, 4294967297];
);
