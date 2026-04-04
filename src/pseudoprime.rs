use crate::utils::{divisibility::is_prime, exp_by_squaring::pow_mod};

/// The Fermat pseudoprimes to a given base.
/// For a base, b, the non-prime positive integers, n, for which the following holds (b^(n-1)) % n = 1
pub struct FermatPseudoprimes {
    ctr: u64,
    base: u64,
}

impl FermatPseudoprimes {
    /// Omly u64 output is supported.
    pub fn new(base: u64) -> Self {
        Self { ctr: 2, base }
    }
}

impl Iterator for FermatPseudoprimes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.ctr += 1;

            // Exclude values that are not co-prime to the base
            if self.ctr % self.base == 0 {
                continue;
            }

            // Exclude primes
            if is_prime(self.ctr) {
                continue;
            }

            if pow_mod(self.base, self.ctr - 1, self.ctr) == 1 {
                break;
            }
        }

        Some(self.ctr)
    }
}

/// The Euler pseudoprimes to a given base.
/// For a base, b, the odd non-prime positive integers, n, for which the following holds (b^((n-1)/2)) % n = (1 or n-1)
pub struct EulerPseudoprimes {
    ctr: u64,
    base: u64,
}

impl EulerPseudoprimes {
    /// Omly u64 output is supported.
    pub fn new(base: u64) -> Self {
        Self { ctr: 3, base }
    }
}

impl Iterator for EulerPseudoprimes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Odd naturals only
            self.ctr += 2;

            // Exclude values that are not co-prime to the base
            if self.ctr % self.base == 0 {
                continue;
            }

            // Exclude primes
            if is_prime(self.ctr) {
                continue;
            }

            let p = pow_mod(self.base, (self.ctr - 1) / 2, self.ctr);

            if p == 1 || p == self.ctr - 1 {
                break;
            }
        }

        Some(self.ctr)
    }
}

crate::check_sequences!(
    FermatPseudoprimes::new(2), [341, 561, 645, 1105, 1387, 1729, 1905, 2047, 2465, 2701, 2821];
    FermatPseudoprimes::new(3), [91, 121, 286, 671, 703, 949, 1105, 1541, 1729, 1891, 2465, 2665];
    EulerPseudoprimes::new(2), [341, 561, 1105, 1729, 1905];
    EulerPseudoprimes::new(3), [121, 703, 1541, 1729, 1891];
);
