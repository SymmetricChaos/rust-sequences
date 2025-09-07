use num::{BigInt, One};

/// The odious numbers, those having an odd number of bits set in their binary representation.
/// 1, 2, 4, 7, 8, 11, 13, 14, 16, 19...
pub struct Odious {
    ctr: BigInt,
}

impl Odious {
    pub fn new() -> Self {
        Self { ctr: BigInt::one() }
    }
}

impl Iterator for Odious {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut odious = true;
            for i in 0..self.ctr.bits() {
                odious ^= self.ctr.bit(i);
            }

            if odious {
                self.ctr += 1;
            } else {
                let out = self.ctr.clone();
                self.ctr += 1;
                return Some(out);
            }
        }
    }
}

crate::check_iteration_times!(
    Odious::new(), 1_100_000;
);


crate::check_sequences!(
    Odious::new(), 0, 10, [1, 2, 4, 7, 8, 11, 13, 14, 16, 19];
);
