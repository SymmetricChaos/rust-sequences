use num::{BigInt, CheckedAdd, Integer};

/// The Perrin sequence. Defined by the recurrence P(n) = P(n-2) + P(n-3) with starting terms 3, 0, 2. Same recurrence as the Padovan sequence.
///
/// 3, 0, 2, 3, 2, 5, 5, 7, 10, 12, 17, 22...
pub struct Perrin<T> {
    n0: T,
    n1: T,
    n2: T,
}

impl<T: Integer> Perrin<T> {
    pub fn new() -> Self {
        Self {
            n0: T::one() + T::one() + T::one(),
            n1: T::zero(),
            n2: T::one() + T::one(),
        }
    }
}

impl Perrin<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Integer + Clone + CheckedAdd> Iterator for Perrin<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.n0.clone();

        let n3 = self.n0.checked_add(&self.n1)?;
        self.n0 = self.n1.clone();
        self.n1 = self.n2.clone();
        self.n2 = n3;

        Some(out)
    }
}

/// The Padovan sequence. Defined by the recurrence P(n) = P(n-2) + P(n-3) with starting terms 1, 1, 1. Same recurrence as the Perrin sequence.
/// 1, 1, 1, 2, 2, 3, 4, 5, 7, 9, 12, 16...
pub struct Padovan<T> {
    n0: T,
    n1: T,
    n2: T,
}

impl<T: Integer> Padovan<T> {
    pub fn new() -> Self {
        Self {
            n0: T::one(),
            n1: T::one(),
            n2: T::one(),
        }
    }
}

impl Padovan<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Integer + Clone + CheckedAdd> Iterator for Padovan<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.n0.clone();

        let n3 = self.n0.checked_add(&self.n1)?;
        self.n0 = self.n1.clone();
        self.n1 = self.n2.clone();
        self.n2 = n3;

        Some(out)
    }
}

crate::check_sequences!(
    Perrin::<i32>::new(), [3, 0, 2, 3, 2, 5, 5, 7, 10, 12, 17, 22, 29, 39, 51, 68, 90, 119, 158, 209, 277, 367, 486, 644, 853, 1130, 1497, 1983, 2627, 3480, 4610, 6107, 8090, 10717, 14197, 18807, 24914, 33004, 43721, 57918, 76725, 101639, 134643, 178364, 236282, 313007, 414646, 549289];
    Padovan::<i32>::new(), [1, 1, 1, 2, 2, 3, 4, 5, 7, 9, 12, 16, 21, 28, 37, 49, 65, 86, 114, 151, 200, 265, 351, 465, 616, 816, 1081, 1432, 1897, 2513, 3329, 4410, 5842, 7739, 10252, 13581, 17991, 23833, 31572, 41824, 55405, 73396, 97229, 128801, 170625, 226030, 299426];
);
