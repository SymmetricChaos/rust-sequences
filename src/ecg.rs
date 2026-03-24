use std::collections::BTreeSet;

use num::integer::gcd;

/// The ECG sequence.
pub struct Ecg {
    used: BTreeSet<u64>, // Checking this becomes much faster than checking a Vec after a few hundred terms
    last: u64,
}

impl Ecg {
    pub fn new() -> Self {
        Self {
            used: BTreeSet::new(),
            last: 0,
        }
    }
}

impl Iterator for Ecg {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.used.len() == 0 {
            self.used.insert(1);
            self.last = 1;
            return Some(1);
        }
        if self.used.len() == 1 {
            self.used.insert(2);
            self.last = 2;
            return Some(2);
        }

        let mut ctr = 2;
        loop {
            ctr += 1;
            if !self.used.contains(&ctr) {
                if gcd(ctr, self.last) != 1 {
                    self.used.insert(ctr);
                    self.last = ctr;
                    return Some(ctr);
                }
            }
        }
    }
}

crate::check_iteration_times!(
    Ecg::new(), 100;
    Ecg::new(), 1_000;
    Ecg::new(), 10_000;
);

crate::check_sequences!(
    Ecg::new(), [1, 2, 4, 6, 3, 9, 12, 8, 10, 5, 15, 18, 14, 7, 21, 24, 16, 20, 22, 11, 33, 27, 30, 25, 35, 28, 26, 13, 39, 36, 32, 34, 17, 51, 42, 38, 19, 57, 45, 40, 44, 46, 23, 69, 48, 50, 52, 54, 56, 49, 63, 60, 55];
);
