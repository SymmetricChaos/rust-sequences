use crate::utils::{divisibility::is_prime, exp_by_squaring::pow_mod};

pub struct Fermat2 {
    ctr: u64,
}

impl Fermat2 {
    pub fn new() -> Self {
        Self { ctr: 341 }
    }
}

impl Iterator for Fermat2 {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.ctr;

        loop {
            // Skip odds
            self.ctr += 2;

            // Exclude primes
            if is_prime(self.ctr) {
                continue;
            }

            if pow_mod(2, self.ctr - 1, self.ctr) == 1 {
                break;
            }
        }

        Some(out)
    }
}

crate::print_sequences!(
    Fermat2::new(), 5;
);

crate::check_sequences!(
    Fermat2::new(), [341, 561, 645, 1105, 1387];
);
