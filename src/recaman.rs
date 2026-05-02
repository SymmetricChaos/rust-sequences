use num::{BigInt, CheckedAdd, CheckedSub, Integer};

/// Recamán's sequence. Decreases by n unless that number has already appeared or would be negative in which case it increases by n.
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
    Recaman::new_big(), [0, 1, 3, 6, 2, 7, 13, 20, 12, 21, 11, 22, 10, 23, 9, 24, 8, 25, 43, 62, 42, 63, 41, 18, 42, 17, 43, 16, 44, 15, 45, 14, 46, 79, 113, 78, 114, 77, 39, 78, 38, 79, 37, 80, 36, 81, 35, 82, 34, 83, 33, 84, 32, 85, 31, 86, 30, 87, 29, 88, 28, 89, 27, 90, 26, 91, 157, 224, 156, 225, 155];
);
