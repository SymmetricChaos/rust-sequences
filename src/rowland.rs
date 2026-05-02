use num::{BigInt, CheckedAdd, Integer};

/// Rowland's sequence R(n) = R(n-1) + gcd(n, R(n-1)) for some positive integer n.
///
/// For n = 7:
/// 7, 8, 9, 10, 15, 18, 19, 20, 21, 22...
pub struct Rowland<T> {
    value: T,
    ctr: T,
}

impl<T: CheckedAdd + Clone + Integer> Rowland<T> {
    pub fn new(initial: T) -> Self {
        Self {
            value: initial,
            ctr: T::one() + T::one(),
        }
    }
}

impl Rowland<BigInt> {
    pub fn new_big<G>(initial: G) -> Self
    where
        BigInt: From<G>,
    {
        Self::new(BigInt::from(initial))
    }
}

impl<T: CheckedAdd + Clone + Integer> Iterator for Rowland<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.value.clone();
        self.value = self.value.checked_add(&self.ctr.gcd(&self.value))?;
        self.ctr = self.ctr.checked_add(&T::one())?;
        Some(out)
    }
}

/// Rowland's prime generating sequence. The first differences of Rowland's sequence. All terms are either 1 or are prime.
///
/// For n = 7:
/// 1, 1, 1, 5, 3, 1, 1, 1, 1, 11...
pub struct RowlandPrime<T> {
    value: T,
    ctr: T,
}

impl<T: CheckedAdd + Clone + Integer> RowlandPrime<T> {
    pub fn new(initial: T) -> Self {
        Self {
            value: initial,
            ctr: T::one() + T::one(),
        }
    }
}

impl RowlandPrime<BigInt> {
    pub fn new_big<G>(initial: G) -> Self
    where
        BigInt: From<G>,
    {
        Self::new(BigInt::from(initial))
    }
}

impl<T: CheckedAdd + Clone + Integer> Iterator for RowlandPrime<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let t = self.value.clone();
        self.value = self.value.checked_add(&self.ctr.gcd(&self.value))?;
        self.ctr = self.ctr.checked_add(&T::one())?;
        Some(self.value.clone() - t)
    }
}

crate::check_sequences!(
    Rowland::new(7),      [7, 8, 9, 10, 15, 18, 19, 20, 21, 22, 33, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 69, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 141, 144, 145, 150, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168];
    RowlandPrime::new(7), [1, 1, 1, 5, 3, 1, 1, 1, 1, 11, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 23, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 47, 3, 1, 5, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 101, 3, 1, 1];
    RowlandPrime::new(7).filter(|x| *x != 1), [5, 3, 11, 3, 23, 3, 47, 3, 5, 3, 101, 3, 7, 11, 3, 13, 233, 3, 467, 3, 5, 3, 941, 3, 7, 1889, 3, 3779, 3, 7559, 3, 13, 15131, 3, 53, 3, 7, 30323, 3, 60647, 3, 5, 3, 101, 3, 121403, 3, 242807, 3, 5, 3, 19, 7, 5, 3, 47, 3, 37, 5, 3, 17, 3, 199, 53, 3, 29, 3, 486041, 3, 7, 421, 23];
);
