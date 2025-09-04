use num::BigInt;

/// The natural numbers. The non-negative integers.
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

/// The counting numbers. The positive integers.
/// 1, 2, 3, 4, 5, 6, 7, 8, 9, 10...
pub struct Counting {
    ctr: BigInt,
}

impl Counting {
    pub fn new() -> Self {
        Self {
            ctr: BigInt::from(1),
        }
    }
}

impl Iterator for Counting {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.ctr.clone();
        self.ctr += 1;
        Some(out)
    }
}

crate::check_sequences!(
    Natural::new(), 0, 10, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    Counting::new(), 0, 10, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
);
