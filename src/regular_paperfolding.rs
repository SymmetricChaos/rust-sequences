use num::{BigInt, CheckedAdd, CheckedDiv, Integer};

/// The odd part of each positive integer. The value after dividing by the largest power of two that is a factor.
pub struct RegularPaperfolding<T> {
    ctr: T,
    two: T,
}

impl<T: Clone + Integer> RegularPaperfolding<T> {
    pub fn new() -> Self {
        Self {
            ctr: T::zero(),
            two: T::one() + T::one(),
        }
    }
}

impl RegularPaperfolding<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Clone + Integer + CheckedDiv + CheckedAdd> Iterator for RegularPaperfolding<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr = self.ctr.checked_add(&T::one())?;

        let mut n = self.ctr.clone();

        while n.is_even() {
            n = n.checked_div(&self.two).unwrap(); // Can't fail but allows using a reference to divide
        }

        if (n % (T::one() + T::one() + T::one() + T::one())).is_one() {
            Some(T::one())
        } else {
            Some(T::zero())
        }
    }
}

crate::check_sequences!(
    RegularPaperfolding::<u32>::new(), [1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0];
);
