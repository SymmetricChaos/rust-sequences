use num::{BigInt, Zero};

/// The evil numbers, those having an even number of bits set in their binary representation.
/// 0, 3, 5, 6, 9, 10, 12, 15, 17, 18...
pub struct Evil {
    ctr: BigInt,
}

impl Evil {
    pub fn new() -> Self {
        Self {
            ctr: BigInt::zero(),
        }
    }
}

impl Iterator for Evil {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut odious = false;
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

crate::check_sequences!(
    Evil::new(), 0, 10, [0, 3, 5, 6, 9, 10, 12, 15, 17, 18];
);
