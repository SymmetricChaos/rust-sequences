use num::{BigInt, CheckedAdd, One, Zero};

/// The natural numbers. The non-negative integers.
/// 0, 1, 2, 3, 4, 5, 6, 7, 8, 9...
pub struct Naturals<T> {
    ctr: T,
}

impl<T: CheckedAdd + Clone + One + Zero> Naturals<T> {
    pub fn new() -> Self {
        Self { ctr: T::zero() }
    }
}

impl Naturals<BigInt> {
    pub fn new_big() -> Self {
        Self {
            ctr: BigInt::zero(),
        }
    }
}

impl<T: CheckedAdd + Clone + One> Iterator for Naturals<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.ctr.clone();
        self.ctr = self.ctr.checked_add(&T::one())?;
        Some(out)
    }
}

crate::check_sequences!(
    Naturals::new_big(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    Naturals::<u8>::new(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
);
