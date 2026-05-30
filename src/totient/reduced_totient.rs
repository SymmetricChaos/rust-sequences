use crate::{
    Number,
    core::traits::Increment,
    utils::{divisibility::prime_power_factorization, totient::totient},
};
use num::{Integer, integer::lcm};

/// The reduced totient function or Carmichael lambda function. Smallest exponent such that n to that power is congruent to 1 modulo for all coprime numbers.
///
/// ```text
/// 1, 1, 2, 2, 4, 2, 6, 2, 6, 4, 10, 2, 12...
/// ```
pub struct ReducedTotient {
    ctr: Number,
}

impl ReducedTotient {
    pub fn new() -> Self {
        Self { ctr: 0 }
    }

    pub fn nth(n: Number) -> Number {
        if n == 0 {
            return 0;
        }
        if n == 1 || n == 2 || n == 4 {
            return totient(n);
        }

        let ppf = prime_power_factorization(n);

        let mut out = if ppf[0].is_even() {
            if ppf[0] > 4 {
                totient(ppf[0]) / 2
            } else {
                totient(ppf[0])
            }
        } else {
            totient(ppf[0])
        };

        for f in ppf[1..].iter() {
            out = lcm(out, totient(*f))
        }
        out
    }
}

impl Iterator for ReducedTotient {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.incr()?;
        Some(Self::nth(self.ctr))
    }
}

crate::check_sequences!(
    ReducedTotient::new(), [1, 1, 2, 2, 4, 2, 6, 2, 6, 4, 10, 2, 12, 6, 4, 4, 16, 6, 18, 4, 6, 10, 22, 2, 20, 12, 18, 6, 28, 4, 30, 8, 10, 16, 12, 6, 36, 18, 12, 4, 40, 6, 42, 10, 12, 22, 46, 4, 42, 20, 16, 12, 52, 18, 20, 6, 18, 28, 58, 4, 60, 30, 6, 16, 12, 10, 66, 16, 22, 12, 70, 6, 72, 36, 20, 18, 30, 12, 78, 4, 54];
);
