use num::BigInt;

/// The natural numbers.
/// 0, 1, 2, 3, 4, 5, 6, 7, 8, 9...
pub struct Natural {
    ctr: BigInt,
}

impl Natural {
    pub fn new() -> Self {
        Self {
            ctr: BigInt::from(0),
        }
    }
}

impl Iterator for Natural {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.ctr.clone();
        self.ctr += 1;
        Some(out)
    }
}

crate::check_sequences!(
    Natural::new(), 0, 10, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
);
