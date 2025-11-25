use num::{BigInt, CheckedAdd, One};

/// The Leonardo numbers.
/// 1, 1, 3, 5, 9, 15, 25, 41, 67, 109...
pub struct Leonardo<T> {
    a: T,
    b: T,
}

impl<T: CheckedAdd + Clone + One> Leonardo<T> {
    pub fn new() -> Self {
        Self {
            a: T::one(),
            b: T::one(),
        }
    }
}

impl Leonardo<BigInt> {
    pub fn new_big() -> Self {
        Self {
            a: BigInt::one(),
            b: BigInt::one(),
        }
    }
}

impl<T: CheckedAdd + Clone + One> Iterator for Leonardo<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let t = self.a.checked_add(&self.b)?.checked_add(&T::one())?;
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

crate::check_iteration_times!(
    Leonardo::new_big(), 160_000;
);

crate::check_sequences!(
    Leonardo::new_big(), 0, 10, [1, 1, 3, 5, 9, 15, 25, 41, 67, 109];
);
