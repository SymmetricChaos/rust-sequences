use crate::{Number, partition::Partition, utils::divisibility::prime_factorization};
use itertools::Itertools;
use num::CheckedMul;

/// The number of abelian groups of order n for each positive integer n.
///
/// ```text
/// 1, 1, 1, 2, 1, 1, 1, 3, 2, 1, 1, 2, 1, 1, 1, 5, 1, 2, 1, 2, 1, 1, 1...
/// ```
pub struct AbelianGroups<T> {
    ctr: T,
    partition_number: Vec<T>,
    partition_number_generator: Partition<T>,
}

impl AbelianGroups<Number> {
    pub fn new() -> Self {
        Self {
            ctr: 0,
            partition_number: Vec::new(),
            partition_number_generator: Partition::new(),
        }
    }
}

impl Iterator for AbelianGroups<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr += 1;
        if self.ctr < 2 {
            return Some(1);
        }

        let powers = prime_factorization(self.ctr)
            .into_iter()
            .map(|(_prime, power)| power as usize)
            .collect_vec();

        let mut out = 1;

        for power in powers {
            while self.partition_number.len() <= power {
                self.partition_number
                    .push(self.partition_number_generator.next()?);
            }
            out = out.checked_mul(&self.partition_number[power])?;
        }

        Some(out)
    }
}

crate::check_sequences!(
    AbelianGroups::new(), [1, 1, 1, 2, 1, 1, 1, 3, 2, 1, 1, 2, 1, 1, 1, 5, 1, 2, 1, 2, 1, 1, 1, 3, 2, 1, 3, 2, 1, 1, 1, 7, 1, 1, 1, 4, 1, 1, 1, 3, 1, 1, 1, 2, 2, 1, 1, 5, 2, 2, 1, 2, 1, 3];
);

crate::sample_sequences!(
    AbelianGroups::new();
);
