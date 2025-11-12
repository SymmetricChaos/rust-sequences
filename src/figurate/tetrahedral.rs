use num::{BigInt, CheckedAdd, CheckedSub, One, Zero};

/// The tetrahedral numbers.
/// 0, 1, 4, 10, 20, 35, 56, 84, 120, 165...
pub struct Tetrahedral<T> {
    a: T,
    b: T,
    ctr: T,
}

impl<T: Clone + CheckedAdd + CheckedSub + One + Zero> Tetrahedral<T> {
    pub fn new() -> Self {
        Self {
            a: T::zero(),
            b: T::one(),
            ctr: T::one() + T::one(),
        }
    }
}

impl Tetrahedral<BigInt> {
    pub fn new_big() -> Self {
        Self {
            a: BigInt::zero(),
            b: BigInt::one(),
            ctr: BigInt::from(2),
        }
    }
}

impl<T: Clone + CheckedAdd + CheckedSub + One> Iterator for Tetrahedral<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let t = self
            .b
            .checked_add(&self.b)?
            .checked_sub(&self.a)?
            .checked_add(&self.ctr)?;
        self.a = self.b.clone();
        self.b = t;
        self.ctr = self.ctr.checked_add(&T::one())?;
        Some(out)
    }
}

crate::check_sequences!(
    Tetrahedral::new_big(), 0, 10, [0, 1, 4, 10, 20, 35, 56, 84, 120, 165];
);
