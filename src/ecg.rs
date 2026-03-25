use std::collections::BTreeSet;

use num::integer::gcd;

/// The ECG sequence. Starting with 1 and 2 each term is the smallest positive integer not yet used which shares a factor with the previous term.
/// 1, 2, 4, 6, 3, 9, 12, 8, 10, 5, 15, 18, 14, 7, 21, 24
pub struct Ecg {
    used: BTreeSet<u64>, // Checking this becomes much faster than checking a Vec after a few hundred terms
    last: u64,
    initial_ctr: u64,
}

impl Ecg {
    /// Only u64 output is supported.
    pub fn new() -> Self {
        Self {
            used: BTreeSet::from([1, 2]),
            last: 0,
            initial_ctr: 3,
        }
    }
}

impl Iterator for Ecg {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.last == 0 {
            self.last = 1;
            return Some(1);
        }
        if self.last == 1 {
            self.last = 2;
            return Some(2);
        }

        let mut ctr = self.initial_ctr;
        // Must be a way to remove the sequential run of numbers that have all been used
        loop {
            if !self.used.contains(&ctr) {
                if gcd(ctr, self.last) != 1 {
                    self.used.insert(ctr);
                    self.last = ctr;
                    return Some(ctr);
                }
            }
            ctr += 1;
        }
    }
}

crate::check_iteration_times!(
    Ecg::new(), 100;
    Ecg::new(), 1_000;
    Ecg::new(), 10_000;
);

crate::check_sequences!(
    Ecg::new(), [1, 2, 4, 6, 3, 9, 12, 8, 10, 5, 15, 18, 14, 7, 21, 24, 16, 20];
);
