use num::{Integer, Zero, rational::Ratio};

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
/// d(p) = 1 (for all primes p)
/// d(mn) = d(m)n + d(n)m (for all naturals m, n)
pub struct ArithmeticDerivative {
    ctr: u64,
}

impl ArithmeticDerivative {
    /// Values are alwaus returned as u64 due to reliance on the prime_factorization function.
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

/// The arithmetic derivatives of the positive rational numbers ordered by antidiagonals.
/// Values are given as i64 because these values may be negative
pub struct ArithmeticDerivativeRational {
    numer: u64,
    denom: u64,
    row: u64,
}

impl ArithmeticDerivativeRational {
    pub fn new() -> Self {
        Self {
            numer: 1,
            denom: 1,
            row: 1,
        }
    }
}

impl Iterator for ArithmeticDerivativeRational {
    type Item = Ratio<i64>;

    fn next(&mut self) -> Option<Self::Item> {
        let a: i64 = arith_deriv(self.numer)
            .checked_mul(self.denom)?
            .try_into()
            .ok()?;
        let b: i64 = arith_deriv(self.denom)
            .checked_mul(self.numer)?
            .try_into()
            .ok()?;
        let n: i64 = self.numer.checked_mul(self.numer)?.try_into().ok()?;

        let out = Ratio::new(a - b, n);
        loop {
            self.numer = self.numer.checked_sub(1)?;
            self.denom = self.denom.checked_add(1)?;
            if self.numer.is_zero() {
                self.row = self.row.checked_add(1)?;
                self.numer = self.row.clone();
                self.denom = 1;
            }
            if self.numer.gcd(&self.denom) == 1 {
                break;
            }
        }

        Some(out)
    }
}

crate::print_values!(
    ArithmeticDerivativeRational::new(), 0, 10;
);

crate::check_sequences!(
    ArithmeticDerivative::new(), 0, 14, [0, 0, 1, 1, 4, 1, 5, 1, 12, 6, 7, 1, 16, 1, 9];
);
