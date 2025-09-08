use num::{BigInt, One};

/// The Catalan numbers.
/// 1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862...
pub struct Catalan {
    val: BigInt,
    ctr: BigInt,
}

impl Catalan {
    pub fn new() -> Self {
        Self {
            val: BigInt::one(),
            ctr: BigInt::one(),
        }
    }
}

impl Iterator for Catalan {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = (((2 * &self.ctr - 1) * 2) * &self.val) / (&self.ctr + 1);
        self.ctr += 1;
        Some(out)
    }
}

crate::check_iteration_times!(
    Catalan::new(), 60_000;
);

crate::check_sequences!(
    Catalan::new(), 0, 10, [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];
);
