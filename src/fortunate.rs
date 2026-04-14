use crate::{core::primes::Primorial, utils::divisibility::is_prime};
use std::iter::Skip;

pub struct Fortunate {
    primorials: Skip<Primorial<u64>>,
}

impl Fortunate {
    pub fn new() -> Self {
        Self {
            primorials: Primorial::new().skip(1),
        }
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
    Fortunate::new(), [3, 5, 7, 13, 23, 17, 19, 23, 37, 61, 67, 61, 71, 47];
);
