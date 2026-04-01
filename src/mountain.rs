use num::{BigInt, Integer};

struct SingleMountain<T> {
    n: T,
    max: T,
    cur: T,
    increase: bool,
}

impl<T: Clone + Integer> SingleMountain<T> {
    fn new(n: T) -> Self {
        Self {
            n: n.clone(),
            max: n.clone() + n.clone(),
            cur: n,
            increase: true,
        }
    }
}

impl<T: Clone + Integer> Iterator for SingleMountain<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.cur.clone();

        if out == self.max {
            self.increase = false
        }
        if !self.increase && out < self.n {
            return None;
        }

        if self.increase {
            self.cur = self.cur.clone() + T::one();
        } else {
            self.cur = self.cur.clone() - T::one();
        }

        Some(out)
    }
}

/// The mountain sequence.
///
/// 1, 2, 1, 2, 3, 4, 3, 2, 1, 2, 1, 2, 3, 4, 3, 2...
pub struct Mountain<T> {
    current_mountian: SingleMountain<T>,
    terms: Vec<T>,
    idx: usize,
}

impl<T: Clone + Integer> Mountain<T> {
    pub fn new() -> Self {
        Self {
            current_mountian: SingleMountain::new(T::one()),
            terms: Vec::new(),
            idx: 0,
        }
    }

    pub fn mountain(n: T) -> Vec<T> {
        let mut m = Vec::new();
        m.push(n);

        m
    }
}

impl Mountain<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Clone + Integer> Iterator for Mountain<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = match self.current_mountian.next() {
            Some(n) => n,
            None => {
                self.idx = self.idx.checked_add(1)?;
                self.current_mountian = SingleMountain::new(self.terms[self.idx].clone());
                self.current_mountian.next()?
            }
        };

        self.terms.push(out.clone());
        Some(out)
    }
}

crate::check_sequences!(
    Mountain::<u32>::new(), [1, 2, 1, 2, 3, 4, 3, 2, 1, 2, 1, 2, 3, 4, 3, 2, 3, 4, 5, 6, 5, 4, 3, 4, 5, 6, 7, 8, 7, 6, 5, 4, 3, 4, 5, 6, 5, 4, 3, 2, 3, 4, 3, 2, 1, 2, 1, 2, 3, 4, 3, 2, 1, 2, 1, 2, 3, 4, 3, 2, 3, 4, 5, 6, 5, 4, 3, 4, 5, 6, 7, 8, 7, 6, 5, 4, 3, 4, 5, 6, 5];
);
