use num::{BigInt, CheckedAdd, One};

/// The Lucas numbers. Defined by the same recurrence as the Fibonacci numbers but starting with 2, 1 rather than 1, 1.
/// 2, 1, 3, 4, 7, 11, 18, 29, 47, 76...
pub struct Lucas<T> {
    a: T,
    b: T,
}

impl<T: CheckedAdd + Clone + One> Lucas<T> {
    pub fn new() -> Self {
        Self {
            a: T::one() + T::one(),
            b: T::one(),
        }
    }
}

impl Lucas<BigInt> {
    pub fn new_big() -> Self {
        Self {
            a: BigInt::from(2),
            b: BigInt::from(1),
        }
    }
}

impl<T: CheckedAdd + Clone> Iterator for Lucas<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let t = self.a.checked_add(&self.b)?;
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

crate::check_iteration_times!(
    Lucas::new_big(), 157_500;
);

crate::check_sequences!(
    Lucas::new_big(), [2, 1, 3, 4, 7, 11, 18, 29, 47, 76];
);
