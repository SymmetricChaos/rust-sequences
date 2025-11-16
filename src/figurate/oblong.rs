use num::{BigInt, CheckedAdd, One, Zero};

/// The oblong numbers.
/// 0, 2, 6, 12, 20, 30, 42, 56, 72, 90...
pub struct Oblong<T> {
    val: T,
    ctr: T,
}

impl<T: CheckedAdd + Clone + One + Zero> Oblong<T> {
    pub fn new() -> Self {
        Self {
            val: T::zero(),
            ctr: T::one() + T::one(),
        }
    }
}

impl Oblong<BigInt> {
    pub fn new_big() -> Self {
        Self {
            val: BigInt::from(0),
            ctr: BigInt::from(2),
        }
    }

    pub fn nth<T>(n: T) -> BigInt
    where
        BigInt: From<T>,
    {
        let n = BigInt::from(n);
        &n * (&n + 1)
    }
}

impl<T: CheckedAdd + Clone + One> Iterator for Oblong<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.val.checked_add(&self.ctr)?;
        self.ctr = self.ctr.checked_add(&(T::one() + T::one()))?;
        Some(out)
    }
}

crate::check_iteration_times!(
    Oblong::new_big(), 3_200_000;
);

crate::check_sequences!(
    Oblong::new_big(), 0, 10, [0, 2, 6, 12, 20, 30, 42, 56, 72, 90];
    Oblong::<i8>::new(), 0, 10, [0, 2, 6, 12, 20, 30, 42, 56, 72, 90];
);
