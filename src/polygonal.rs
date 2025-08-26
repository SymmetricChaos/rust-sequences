use num::{BigInt, One, Zero};

/// The polygonal numbers with selectable number order
pub struct Polygonal {
    val: BigInt,
    ctr: BigInt,
    inc: BigInt,
}

impl Polygonal {
    /// n = 0 -> natural numbers
    /// n = 1 -> triangular numbers
    /// n = 2 -> square numbers
    /// and then higher order polygonal numbers
    pub fn new(n: u32) -> Self {
        Self {
            val: BigInt::zero(),
            ctr: BigInt::one(),
            inc: BigInt::from(n),
        }
    }
}

impl Iterator for Polygonal {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.val.checked_add(&self.ctr)?;
        self.ctr = self.ctr.checked_add(&self.inc)?;
        Some(out)
    }
}

crate::print_a_few!(
    super::Polygonal::new(0), 0, 10;
    super::Polygonal::new(1), 0, 10;
    super::Polygonal::new(2), 0, 10;
    super::Polygonal::new(3), 0, 10;
);
