use num::{BigInt, One, Zero};

pub struct Polygonal {
    val: BigInt,
    ctr: BigInt,
    inc: BigInt,
}

impl Polygonal {
    /// Create a sequence of polygonal numbers starting from zero.
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

mod tests {

    #[test]
    fn seq() {
        use super::Polygonal;
        let x = Polygonal::new(2);
        for n in x.skip(0).take(10) {
            println!("{n}")
        }
    }
}
