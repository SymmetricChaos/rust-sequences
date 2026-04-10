use num::{BigInt, CheckedAdd, CheckedSub, Integer};

/// Recamán's sequence. Decreases by n unless that number has already appeared (or would be negative) in which case it increases by n.
///
/// 0, 1, 3, 6, 2, 7, 13, 20, 12, 21, 11, 22...
pub struct Recaman<T> {
    n: T,
    ctr: T,
    prev: Vec<T>,
}

impl<T: Clone + CheckedAdd + CheckedSub + Integer> Recaman<T> {
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
        Self::new()
    }
}

impl<T: Clone + CheckedAdd + CheckedSub + Integer> Iterator for Recaman<T> {
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
    Recaman::new_big(), [0, 1, 3, 6, 2, 7, 13, 20, 12, 21, 11, 22, 10, 23, 9, 24, 8, 25, 43, 62];
);
