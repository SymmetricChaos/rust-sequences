use crate::{core::traits::Increment, utils::divisibility::prime_factorization};

/// The Blum integers. Natural numbers of the form p*q where p and q are primes congruent to 3 modulo 4 and p is not equal to q. They are relevant to the Blum-Blum-Shub PRNG.
///
/// 21, 33, 57, 69, 77, 93, 129, 133, 141...
pub struct Blum {
    ctr: u64,
}

impl Blum {
    pub fn new() -> Self {
        Self { ctr: 20 }
    }
}

impl Iterator for Blum {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.ctr.incr()?;
            let f = prime_factorization(self.ctr);
            if f.len() == 2 && f[0].1 == 1 && f[1].1 == 1 {
                if f[0].0 % 4 == 3 && f[1].0 % 4 == 3 {
                    return Some(self.ctr);
                }
            }
        }
    }
}

crate::check_sequences!(
    Blum::new(), [21, 33, 57, 69, 77, 93, 129, 133, 141, 161, 177, 201, 209, 213, 217, 237, 249, 253, 301, 309, 321, 329, 341, 381, 393, 413, 417, 437, 453, 469, 473, 489, 497, 501, 517, 537, 553, 573, 581, 589, 597, 633, 649, 669, 681, 713, 717, 721, 737, 749, 753, 781, 789];
);
