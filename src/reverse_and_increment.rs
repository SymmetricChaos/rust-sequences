use num::{BigInt, CheckedAdd, Integer};

pub struct ReverseAndIncrement<T> {
    terms: Vec<T>,
    idx: usize,
}

impl<T: CheckedAdd + Clone + Integer> ReverseAndIncrement<T> {
    pub fn new() -> Self {
        Self {
            terms: vec![T::one()],
            idx: 0,
        }
    }
}

impl ReverseAndIncrement<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: CheckedAdd + Clone + Integer> Iterator for ReverseAndIncrement<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.terms[self.idx].clone();

        // self.term will run out of space before this overflows so no check is needed
        self.idx += 1;

        if self.idx == self.terms.len() {
            for i in self.terms.clone().into_iter().rev() {
                self.terms.push(i.checked_add(&T::one())?);
            }
        }

        Some(out)
    }
}

crate::check_sequences!(
    ReverseAndIncrement::<i32>::new(), [1, 2, 3, 2, 3, 4, 3, 2, 3, 4, 5, 4, 3, 4, 3, 2, 3, 4, 5, 4, 5, 6, 5, 4, 3, 4, 5, 4, 3, 4, 3, 2, 3, 4, 5, 4, 5, 6, 5, 4, 5, 6, 7, 6, 5, 6, 5, 4, 3, 4, 5, 4, 5, 6, 5, 4, 3, 4, 5, 4, 3, 4, 3, 2, 3, 4, 5, 4, 5, 6, 5, 4, 5, 6, 7, 6, 5, 6, 5, 4, 5, 6, 7, 6, 7, 8, 7, 6, 5, 6, 7, 6, 5, 6, 5, 4, 3, 4, 5, 4, 5, 6];
);
