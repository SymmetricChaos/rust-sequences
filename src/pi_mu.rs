use num::{BigInt, CheckedAdd, Integer, Signed};

/// Starting with 1 the sequence is extended by repeatedly taking the terms generated so far and appending them in reverse order, each incremented by one.
///
/// 1, 2, 3, 2, 3, 4, 3, 2, 3, 4...
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

/// The pi-mu sequences. The signed differences of the the reverse and increment sequence.
///
/// 1, 1, -1, 1, 1, -1, -1, 1, 1, 1...
pub struct PiMu<T> {
    prev: T,
    s: ReverseAndIncrement<T>,
}

impl<T: CheckedAdd + Clone + Integer + Signed> PiMu<T> {
    pub fn new() -> Self {
        let mut s = ReverseAndIncrement::new();
        let prev = s.next().unwrap();
        Self { prev, s }
    }
}

impl PiMu<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: CheckedAdd + Clone + Integer + Signed> Iterator for PiMu<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.s.next()?;
        let out = n.clone() - self.prev.clone();
        self.prev = n;
        Some(out)
    }
}

crate::check_sequences!(
    ReverseAndIncrement::<i32>::new(), [1, 2, 3, 2, 3, 4, 3, 2, 3, 4, 5, 4, 3, 4, 3, 2, 3, 4, 5, 4, 5, 6, 5, 4, 3, 4, 5, 4, 3, 4, 3, 2, 3, 4, 5, 4, 5, 6, 5, 4, 5, 6, 7, 6, 5, 6, 5, 4, 3, 4, 5, 4, 5, 6, 5, 4, 3, 4, 5, 4, 3, 4, 3, 2, 3, 4, 5, 4, 5, 6, 5, 4, 5, 6, 7, 6, 5, 6, 5, 4, 5, 6, 7, 6, 7, 8, 7, 6, 5, 6, 7, 6, 5, 6, 5, 4, 3, 4, 5, 4, 5, 6];
    PiMu::<i32>::new(), [1, 1, -1, 1, 1, -1, -1, 1, 1, 1, -1, -1, 1, -1, -1, 1, 1, 1, -1, 1, 1, -1, -1, -1, 1, 1, -1, -1, 1, -1, -1];
);
