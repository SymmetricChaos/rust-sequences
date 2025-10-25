use num::{BigInt, CheckedAdd, One, Zero};

/// The Pell numbers.
/// 0, 1, 2, 5, 12, 29, 70, 169, 408, 985...
pub struct Pell<T> {
    a: T,
    b: T,
}

impl<T: CheckedAdd + Clone + One + Zero> Pell<T> {
    pub fn new() -> Self {
        Self {
            a: T::zero(),
            b: T::one(),
        }
    }
}

impl Pell<BigInt> {
    pub fn new_big() -> Self {
        Self {
            a: BigInt::zero(),
            b: BigInt::one(),
        }
    }
}

impl<T: CheckedAdd + Clone + One + Zero> Iterator for Pell<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let t = self.a.checked_add(&(self.b.checked_add(&self.b)?))?;
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

/// The companion Pell numbers.
/// 2, 2, 6, 14, 34, 82, 198, 478, 1154, 2786...
pub struct CompanionPell<T> {
    a: T,
    b: T,
}

impl<T: CheckedAdd + Clone + One + Zero> CompanionPell<T> {
    pub fn new() -> Self {
        Self {
            a: T::one() + T::one(),
            b: T::one() + T::one(),
        }
    }
}

impl CompanionPell<BigInt> {
    pub fn new_big() -> Self {
        Self {
            a: BigInt::one() + BigInt::one(),
            b: BigInt::one() + BigInt::one(),
        }
    }
}

impl<T: CheckedAdd + Clone + One + Zero> Iterator for CompanionPell<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let t = self.a.checked_add(&(self.b.checked_add(&self.b)?))?;
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

crate::check_iteration_times!(
    Pell::new_big(), 90_000;
);

crate::check_sequences!(
    Pell::new_big(), 0, 10, [0, 1, 2, 5, 12, 29, 70, 169, 408, 985];
    CompanionPell::new_big(), 0, 10, [2, 2, 6, 14, 34, 82, 198, 478, 1154, 2786];
);
