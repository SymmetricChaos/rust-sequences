use num::{BigInt, CheckedAdd, CheckedMul, One};

pub struct Sylvester<T> {
    factors: Vec<T>,
}

impl<T: Clone + One + CheckedMul + CheckedAdd> Sylvester<T> {
    pub fn new() -> Self {
        Self {
            factors: vec![T::one()],
        }
    }
}

impl Sylvester<BigInt> {
    pub fn new_big() -> Self {
        Self {
            factors: vec![BigInt::one()],
        }
    }
}

impl<T: Clone + One + CheckedMul + CheckedAdd> Iterator for Sylvester<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut out = T::one();
        for f in self.factors.iter() {
            out = out.checked_mul(f)?;
        }
        out = out.checked_add(&T::one())?;

        self.factors.push(out.clone());
        Some(out)
    }
}

crate::check_sequences!(
    Sylvester::new_big(), 0, 6, [2, 3, 7, 43, 1807, 3263443];
);
