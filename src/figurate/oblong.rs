use num::{BigInt, CheckedAdd, One, Zero};

/// The oblong or pronic numbers. For each non-negatie integer n the term is n * (n+1).
///
/// 0, 2, 6, 12, 20, 30, 42, 56, 72, 90...
pub struct Oblong<T> {
    a: T,
    b: T,
}

impl<T: CheckedAdd + Clone + One + Zero> Oblong<T> {
    pub fn new() -> Self {
        Self {
            a: T::zero(),
            b: T::one() + T::one(),
        }
    }
}

impl Oblong<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }

    pub fn nth<T>(n: T) -> BigInt
    where
        BigInt: From<T>,
    {
        let n = &BigInt::from(n);
        n * (n + 1)
    }
}

impl<T: CheckedAdd + Clone + One> Iterator for Oblong<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        self.a = self.a.checked_add(&self.b)?;
        self.b = self.b.checked_add(&(T::one() + T::one()))?;
        Some(out)
    }
}

crate::check_iteration_times!(
    Oblong::new_big(), 3_200_000;
);

crate::check_sequences!(
    Oblong::new_big(), [0, 2, 6, 12, 20, 30, 42, 56, 72, 90];
    Oblong::<i8>::new(), [0, 2, 6, 12, 20, 30, 42, 56, 72, 90];
);
