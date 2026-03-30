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
    Rowland::new(7), [7, 8, 9, 10, 15, 18, 19, 20, 21, 22];
    RowlandPrime::new(7), [1, 1, 1, 5, 3, 1, 1, 1, 1, 11];
    RowlandPrime::new(7).filter(|x| *x != 1), [5, 3, 11, 3, 23, 3, 47];
);
