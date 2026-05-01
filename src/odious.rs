use num::{BigInt, Integer, One};

/// The odious numbers, those having an odd number of bits set in their binary representation. So called as a pun on the evil numbers.
///
/// 1, 2, 4, 7, 8, 11, 13, 14, 16, 19...
pub struct Odious {
    ctr: BigInt,
}

impl Odious {
    pub fn new_big() -> Self {
        Self { ctr: BigInt::one() }
    }
}

impl Iterator for Odious {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.ctr.magnitude().count_ones().is_odd() {
                let out = self.ctr.clone();
                self.ctr.inc();
                return Some(out);
            }
            self.ctr.inc();
        }
    }
}

crate::check_iteration_times!(
    Odious::new_big(), 1_000_000;
);

crate::check_sequences!(
    Odious::new_big(), [1, 2, 4, 7, 8, 11, 13, 14, 16, 19];
);
