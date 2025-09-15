use num::{BigInt, CheckedAdd, One, PrimInt, Zero};

/// The natural numbers. The non-negative integers.
/// 0, 1, 2, 3, 4, 5, 6, 7, 8, 9...
pub struct Natural<T> {
    ctr: T,
}

impl<T: PrimInt> Natural<T> {
    pub fn new_prim() -> Self {
        Self { ctr: T::zero() }
    }
}

impl Natural<BigInt> {
    pub fn new() -> Self {
        Self {
            ctr: BigInt::zero(),
        }
    }
}

impl<T: Clone + CheckedAdd + One> Iterator for Natural<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.ctr.clone();
        self.ctr = self.ctr.checked_add(&T::one())?;
        Some(out)
    }
}

crate::check_sequences!(
    Natural::new(), 0, 10, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    Natural::<u8>::new_prim(), 0, 10, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
);
