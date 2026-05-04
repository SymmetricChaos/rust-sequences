use std::collections::BTreeSet;

use num::integer::gcd;

/// The ECG sequence. Starting with 1 and 2 each term is the smallest positive integer not yet used which shares a divisor with the previous term.
///
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
            initial_ctr: 1,
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

        // Trim the btree and advance the initial counter to repeating unnecessary work
        // This slightly slows down the iterator for the first few hundred values but hugely speeds it up after the first 1000
        if self.used.contains(&self.initial_ctr) {
            loop {
                if self.used.contains(&(self.initial_ctr + 1)) {
                    self.used.remove(&self.initial_ctr);
                    self.initial_ctr += 1;
                } else {
                    break;
                }
            }
        }

        let mut ctr = self.initial_ctr;

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
    Ecg::new(), [100, 1_000, 10_000];
);

crate::check_sequences!(
    Ecg::new(), [1, 2, 4, 6, 3, 9, 12, 8, 10, 5, 15, 18, 14, 7, 21, 24, 16, 20, 22, 11, 33, 27, 30, 25, 35, 28, 26, 13, 39, 36, 32, 34, 17, 51, 42, 38, 19, 57, 45, 40, 44, 46, 23, 69, 48, 50, 52, 54, 56, 49, 63, 60, 55, 65, 70, 58, 29, 87, 66, 62, 31, 93, 72, 64, 68, 74, 37, 111, 75, 78, 76, 80, 82];
);
