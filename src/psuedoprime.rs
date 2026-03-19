use crate::utils::{divisibility::is_prime, exp_by_squaring::pow_mod};

/// The Fermat pseudoprimes to a given base.
/// For a base, b, the non-prime numbers, n, for which the following holds (b^(n-1)) % n = 1
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
            // If the base == 2 this can be 2 in order to skip evens
            // A number divisible by the base is never a pseudoprime to that base
            self.ctr += 1;

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
/// For a base, b, the odd non-prime numbers, n, for which the following holds (b^((n-1)/2)) % n = (1 or n-1)
/// This condition is twice as strong as for a Fermat pseudoprime.
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
    FermatPseudoprimes::new(2), [341, 561, 645, 1105, 1387];
    FermatPseudoprimes::new(3), [91, 121, 286, 671, 703];
    EulerPseudoprimes::new(2), [341, 561, 1105, 1729, 1905];
    EulerPseudoprimes::new(3), [121, 703, 1541, 1729, 1891];
);
