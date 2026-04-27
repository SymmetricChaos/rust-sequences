use crate::{core::primes::Primorial, utils::divisibility::is_prime};

/// The fortunate numbers.
///
/// 3, 5, 7, 13, 23, 17, 19, 23, 37, 61, 67, 61, 71, 47...
pub struct Fortunate {
    primorials: Primorial<u64>,
}

impl Fortunate {
    /// Only u64 output is supported.
    pub fn new() -> Self {
        let mut primorials = Primorial::new();
        primorials.next();
        Self { primorials }
    }
}

impl Iterator for Fortunate {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let p = self.primorials.next()?;
        let mut n = 2;
        loop {
            if is_prime(p.checked_add(n)?) {
                return Some(n);
            }
            n += 1;
        }
    }
}

crate::check_sequences!(
    Fortunate::new(),
    [3, 5, 7, 13, 23, 17, 19, 23, 37, 61, 67, 61, 71, 47]; //, 107, 59, 61, 109, 89, 103, 79, 151, 197, 101, 103, 233, 223, 127, 223, 191, 163, 229, 643, 239, 157, 167, 439, 239, 199, 191, 199, 383, 233, 751, 313, 773, 607, 313, 383, 293, 443, 331, 283, 277, 271, 401, 307, 331];
);
