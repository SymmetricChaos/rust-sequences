use num::{BigInt, One, Zero};

use crate::check_sequences;

pub fn unsigned_stirling_first(n: &BigInt, k: &BigInt) -> BigInt {
    if n == k {
        return BigInt::one();
    }
    if k.is_zero() || n.is_zero() {
        return BigInt::zero();
    }

    (n - 1) * unsigned_stirling_first(&(n - 1), &k) + unsigned_stirling_first(&(n - 1), &(k - 1))
}

/// Unsigned Stirling numbers of the first kind.
pub struct UnsignedStirlingFirst {
    n: BigInt,
    k: BigInt,
}

impl UnsignedStirlingFirst {
    pub fn new() -> Self {
        Self {
            n: BigInt::zero(),
            k: BigInt::zero(),
        }
    }
}

impl Iterator for UnsignedStirlingFirst {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = Some(unsigned_stirling_first(&self.n, &self.k));
        println!("{} {} = {} ", self.n, self.k, out.clone().unwrap());
        if self.n == self.k {
            self.n += 1;
            self.k = BigInt::zero();
        } else {
            self.k += 1;
        }
        out
    }
}

pub struct StirlingSecond {}

check_sequences!(
    UnsignedStirlingFirst::new(), 0, 20, [1, 0, 1, 0, 1, 1, 0, 2, 3, 1, 0, 6, 11, 6, 1, 0, 24, 50, 35, 10];
);
