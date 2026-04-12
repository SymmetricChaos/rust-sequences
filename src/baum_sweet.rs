use num::{BigInt, Integer, One, Zero};

/// The Baum-Sweet sequence.  Characteristic function of positive intgers which have binary expansions that never contain an odd number of sequential 0s.
///
/// 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1..
pub struct BaumSweet<T> {
    terms: Vec<T>,
    ctr: usize,
}

impl<T: Clone + One + Zero> BaumSweet<T> {
    pub fn new() -> Self {
        Self {
            terms: vec![T::one()],
            ctr: 0,
        }
    }
}

impl BaumSweet<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Clone + One + Zero> Iterator for BaumSweet<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr += 1;
        let mut n = self.ctr.clone();

        while (n % 4).is_zero() {
            n = n / 4;
        }
        if n.is_even() {
            self.terms.push(T::zero());
        } else {
            self.terms.push(self.terms[(n - 1) / 2].clone());
        }

        let out = self.terms[self.ctr].clone();

        Some(out)
    }
}

crate::check_sequences!(
    BaumSweet::new_big(), [1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1];
);
