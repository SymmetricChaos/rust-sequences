use num::{BigInt, CheckedAdd, One, Zero};

/// The oblong or pronic numbers.
///
/// ```text
/// f(n) = n*(n+1)
/// 0, 2, 6, 12, 20, 30, 42, 56, 72, 90...
/// ```
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
    Oblong::new_big(), [0, 2, 6, 12, 20, 30, 42, 56, 72, 90, 110, 132, 156, 182, 210, 240, 272, 306, 342, 380, 420, 462, 506, 552, 600, 650, 702, 756, 812, 870, 930, 992, 1056, 1122, 1190, 1260, 1332, 1406, 1482, 1560, 1640, 1722, 1806, 1892, 1980, 2070, 2162, 2256, 2352, 2450, 2550];
);
