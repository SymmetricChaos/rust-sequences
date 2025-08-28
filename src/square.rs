use num::BigInt;

/// The triangular numbers.
/// 0, 1, 4, 9, 16, 25, 36, 49, 64, 81...
pub struct Square {
    val: BigInt,
    ctr: BigInt,
}

impl Square {
    pub fn new() -> Self {
        Self {
            val: BigInt::from(0),
            ctr: BigInt::from(1),
        }
    }
}

impl Iterator for Square {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val += &self.ctr;
        self.ctr += 2;
        Some(out)
    }
}

crate::check_sequences!(
    Square::new(), 0, 10, [0, 1, 4, 9, 16, 25, 36, 49, 64, 81];
);
