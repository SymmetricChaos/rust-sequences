use num::{BigInt, CheckedAdd, One, Zero};

/// The triangular numbers.
/// 0, 1, 3, 6, 10, 15, 21, 28, 36, 45...
pub struct Triangular<T> {
    val: T,
    ctr: T,
}

impl<T: Clone + CheckedAdd + One + Zero> Triangular<T> {
    pub fn new() -> Self {
        Self {
            val: T::zero(),
            ctr: T::one(),
        }
    }
}

impl Triangular<BigInt> {
    pub fn new_big() -> Self {
        Self {
            val: BigInt::zero(),
            ctr: BigInt::one(),
        }
    }
}

impl<T: Clone + CheckedAdd + One> Iterator for Triangular<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.val.checked_add(&self.ctr)?;
        self.ctr = self.ctr.checked_add(&T::one())?;
        Some(out)
    }
}

crate::print_values!(
    Triangular::new_big(), 0, 10;
    Triangular::new_big(), 1, 10;
    Triangular::new_big(), 2, 10;
    Triangular::<i32>::new(), 0, 10;
    Triangular::<i32>::new(), 1, 10;
    Triangular::<i32>::new(), 2, 10;
);

crate::check_sequences!(
    Triangular::new_big(), [0, 1, 3, 6, 10, 15, 21, 28, 36, 45];
);
