use num::BigInt;

/// The Catalan numbers.
/// 1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862...
pub struct Catalan {
    val: BigInt,
    ctr: BigInt,
}

impl Catalan {
    pub fn new() -> Self {
        Self {
            val: BigInt::from(1),
            ctr: BigInt::from(1),
        }
    }
}

impl Iterator for Catalan {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = (((2 * &self.ctr - 1) * 2)* &self.val) / (&self.ctr + 1) ;
        self.ctr += 1;
        Some(out)
    }
}

crate::check_sequences!(
    Catalan::new(), 0, 10, [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];
);
