use num::{BigInt, Integer, One, Zero};

//TODO: Is there a reasonable way to make this generic?

/// The evil numbers, those having an even number of bits set in their binary representation. So called as a pun on the odius numbers.
///
/// 0, 3, 5, 6, 9, 10, 12, 15, 17, 18...
pub struct Evil {
    ctr: BigInt,
}

impl Evil {
    pub fn new_big() -> Self {
        Self {
            ctr: BigInt::zero(),
        }
    }
}

impl Iterator for Evil {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.ctr.magnitude().count_ones().is_even() {
                let out = self.ctr.clone();
                self.ctr.inc();
                return Some(out);
            }
            self.ctr.inc();
        }
    }
}

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
    Evil::new_big(), 1_000_000;
    Odious::new_big(), 1_000_000;
);

crate::check_sequences!(
    Evil::new_big(), [0, 3, 5, 6, 9, 10, 12, 15, 17, 18];
    Odious::new_big(), [1, 2, 4, 7, 8, 11, 13, 14, 16, 19];
);
