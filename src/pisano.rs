use crate::{core::traits::Increment, utils::divisibility::prime_power_factorization};
use num::{One, integer::lcm};
use std::collections::HashMap;

/// The Pisano period for each positive integer.
///
/// 1, 3, 8, 6, 20, 24, 16, 12, 24...
pub struct PisanoPeriods {
    ctr: u64,
    prime_power_map: HashMap<u64, u64>,
}

impl PisanoPeriods {
    /// Only u64 output is supported.
    pub fn new() -> Self {
        Self {
            ctr: 0,
            prime_power_map: HashMap::new(),
        }
    }
}

impl Iterator for PisanoPeriods {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.incr()?;
        if self.ctr.is_one() {
            return Some(1);
        }

        // Factor into prime powers
        let ppf = prime_power_factorization(self.ctr);

        // If if is a prime power then its Pisano period is calculated directly and recorded
        if ppf.len() == 1 {
            let mut k = 1;
            let mut x = [1, 1];
            while x != [0, 1] {
                k += 1;
                x = [x[1], (x[0] + x[1]) % self.ctr]
            }
            self.prime_power_map.insert(self.ctr, k);
            return Some(k);
        } else {
            // If it is not a prime power than the Pisano period is calculated from its prime power factorization
            let k = ppf
                .into_iter()
                .fold(1, |acc, e| lcm(acc, self.prime_power_map[&e]));
            return Some(k);
        }
    }
}

crate::check_sequences!(
    PisanoPeriods::new(), [1, 3, 8, 6, 20, 24, 16, 12, 24, 60, 10, 24, 28, 48, 40, 24, 36, 24, 18, 60, 16, 30, 48, 24, 100, 84, 72, 48, 14, 120, 30, 48, 40, 36, 80, 24, 76, 18, 56, 60, 40, 48, 88, 30, 120, 48, 32, 24, 112, 300, 72, 84, 108, 72, 20, 48, 72, 42, 58, 120, 60, 30, 48, 96, 140, 120, 136];
);
