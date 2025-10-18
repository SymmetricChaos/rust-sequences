use num::{BigInt, Zero};

/// The Gray codes, bit sequences each differing by a single bit. A permutation of the integers.
pub struct Gray {
    ctr: BigInt,
}

impl Gray {
    pub fn new_big() -> Self {
        Self {
            ctr: BigInt::zero(),
        }
    }
}

impl Iterator for Gray {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = &self.ctr ^ (&self.ctr >> 1);
        self.ctr += 1;
        Some(out)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.ctr += n;
        let out = &self.ctr ^ (&self.ctr >> 1);
        self.ctr += 1;
        Some(out)
    }
}

crate::check_sequences!(Gray::new_big(), 0, 10, [0, 1, 3, 2, 6, 7, 5, 4, 12, 13]);
