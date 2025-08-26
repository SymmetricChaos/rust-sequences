use num::{CheckedAdd, One, Zero};

pub struct Natural<T> {
    ctr: T,
}

impl<T: CheckedAdd + Clone + Zero> Natural<T> {
    pub fn new() -> Self {
        Self { ctr: T::zero() }
    }
}

impl<T: CheckedAdd + Clone + One> Iterator for Natural<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.ctr.clone();
        self.ctr = self.ctr.checked_add(&T::one())?;
        Some(out)
    }
}
