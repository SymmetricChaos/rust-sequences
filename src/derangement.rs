use num::{BigInt, CheckedAdd, CheckedMul, One, Zero};

// The number of derangements for a set of n elements range over all natural numbers.
pub struct Deramgements<T> {
    a: T,
    b: T,
    ctr: T,
}

impl<T: One + Zero> Deramgements<T> {
    pub fn new() -> Self {
        Self {
            a: T::one(),
            b: T::zero(),
            ctr: T::one(),
        }
    }
}

impl Deramgements<BigInt> {
    pub fn new_big() -> Self {
        Self {
            a: BigInt::one(),
            b: BigInt::zero(),
            ctr: BigInt::one(),
        }
    }
}

impl<T: Clone + CheckedAdd + CheckedMul + One> Iterator for Deramgements<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();

        let next = self.ctr.checked_mul(&self.a.checked_add(&self.b)?)?;
        self.a = self.b.clone();
        self.b = next;
        self.ctr = self.ctr.checked_add(&T::one())?;

        Some(out)
    }
}

crate::check_sequences!(
    Deramgements::<i32>::new(), 0, 10, [1, 0, 1, 2, 9, 44, 265, 1854, 14833, 133496];
);
