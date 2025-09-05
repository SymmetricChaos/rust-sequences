use num::BigInt;

/// The odious numbers.
/// 1, 2, 4, 7, 8, 11, 13, 14, 16, 19...
pub struct Odious {
    ctr: BigInt,
}

impl Odious {
    pub fn new() -> Self {
        Self {
            ctr: BigInt::from(1),
        }
    }
}

impl Iterator for Odious {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        todo!("turns out no.popcount() for BigInt")
    }
}

crate::check_sequences!(
    Odious::new(), 0, 10, [1, 2, 4, 7, 8, 11, 13, 14, 16, 19];
);
