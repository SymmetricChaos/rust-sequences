use num::BigInt;

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
            val: BigInt::from(0),
            ctr: BigInt::from(1),
            inc: BigInt::from(n),
        }
    }
}

impl Iterator for Polygonal {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val += &self.ctr;
        self.ctr += &self.inc;
        Some(out)
    }
}

crate::print_a_few!(
    Polygonal::new(0), 0, 10;
    Polygonal::new(1), 0, 10;
    Polygonal::new(2), 0, 10;
    Polygonal::new(3), 0, 10;
);
