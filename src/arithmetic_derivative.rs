use num::{Zero, rational::Ratio};

use crate::core::primality_utils::prime_factorization;

fn arith_deriv(n: u64) -> u64 {
    let factors = prime_factorization(n);
    if factors.len() == 0 {
        return 0;
    }
    if factors.len() == 1 && factors[0].1 == 1 {
        return 1;
    } else {
        let mut s = Ratio::zero();
        for (prime, mult) in factors {
            s += Ratio::new(mult, prime);
        }
        *(s * Ratio::new_raw(n, 1)).numer()
    }
}

/// The so-called arithmetic derivative of each natural number defined as follows
/// d(0) = 0
/// d(1) = 0
/// d(p) = 1 (for all primes p)
/// d(mn) = d(m)n + d(n)m (for all naturals m, n)
pub struct ArithmeticDerivative {
    ctr: u64,
}

impl ArithmeticDerivative {
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for ArithmeticDerivative {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let out = arith_deriv(self.ctr);
        self.ctr = self.ctr.checked_add(1)?;
        Some(out)
    }
}
