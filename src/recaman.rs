use num::{BigInt, CheckedAdd, CheckedSub, One, Zero};

/// Recamán's sequence
pub struct Recaman<T> {
    n: T,
    ctr: T,
    prev: Vec<T>,
}

impl<T: Clone + CheckedAdd + CheckedSub + PartialOrd + One + Zero> Recaman<T> {
    pub fn new() -> Self {
        Self {
            n: T::zero(),
            ctr: T::one(),
            prev: Vec::new(),
        }
    }
}

impl Recaman<BigInt> {
    pub fn new_big() -> Self {
        Self {
            n: BigInt::zero(),
            ctr: BigInt::one(),
            prev: Vec::new(),
        }
    }
}

impl<T: Clone + CheckedAdd + CheckedSub + PartialOrd + One> Iterator for Recaman<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.n.clone();

        if self.ctr >= self.n {
            self.n = self.n.checked_add(&self.ctr)?
        } else {
            let down = self.n.checked_sub(&self.ctr)?;
            if self.prev.contains(&down) {
                self.n = self.n.checked_add(&self.ctr)?;
            } else {
                self.n = down;
            }
        }

        self.ctr = self.ctr.checked_add(&T::one())?;

        self.prev.push(self.n.clone());

        Some(out)
    }
}

crate::check_sequences!(
    Recaman::new_big(), 0, 20, [0, 1, 3, 6, 2, 7, 13, 20, 12, 21, 11, 22, 10, 23, 9, 24, 8, 25, 43, 62];
);
