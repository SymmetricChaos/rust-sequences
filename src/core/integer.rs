use num::{BigInt, CheckedAdd, Signed};

/// The integers in the canonical ordering.
///
/// 0, 1, -1, 2, -2, 3, -3, 4, -4, 5...
pub struct Integers<T> {
    val: T,
}

impl<T: CheckedAdd + Clone + Signed> Integers<T> {
    pub fn new() -> Self {
        Self { val: T::zero() }
    }
}

impl Integers<BigInt> {
    pub fn new_big() -> Self {
        Self {
            val: BigInt::from(0),
        }
    }
}

impl<T: CheckedAdd + Clone + Signed> Iterator for Integers<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        if self.val.is_positive() {
            self.val = -self.val.clone();
        } else {
            self.val = -self.val.clone();
            self.val = self.val.checked_add(&T::one())?;
        };
        Some(out)
    }
}

crate::check_iteration_times!(
    Integers::new_big(), 4_000_000;
    Integers::<i32>::new(), 4_000_000;
);

crate::check_sequences!(
    Integers::new_big(), [0, 1, -1, 2, -2, 3, -3, 4, -4, 5, -5, 6, -6, 7, -7, 8, -8, 9, -9, 10, -10, 11, -11, 12, -12, 13, -13, 14, -14, 15, -15, 16, -16, 17, -17, 18, -18, 19, -19, 20, -20, 21, -21, 22, -22, 23, -23, 24, -24, 25, -25, 26, -26, 27, -27, 28, -28, 29, -29, 30, -30, 31, -31];
);
