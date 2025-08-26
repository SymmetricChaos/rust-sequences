use num::{BigInt, One, Zero};

pub struct Natural {
    ctr: BigInt,
}

impl Natural {
    pub fn new() -> Self {
        Self {
            ctr: BigInt::zero(),
        }
    }
}

impl Iterator for Natural {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.ctr.clone();
        self.ctr = self.ctr.checked_add(&BigInt::one())?;
        Some(out)
    }
}
