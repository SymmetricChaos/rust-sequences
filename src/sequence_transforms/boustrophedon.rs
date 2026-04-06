use num::{CheckedAdd, Integer};

/// The boustrophedon transform of a sequence
pub struct Boustrophedon<T> {
    iter: Box<dyn Iterator<Item = T>>,
    row: Vec<T>,
}

impl<T: CheckedAdd + Clone> Boustrophedon<T> {
    pub fn new<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            row: vec![iter.next().unwrap()],
            iter: Box::new(iter),
        }
    }
}

impl<T: CheckedAdd + Clone> Iterator for Boustrophedon<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.row.last().unwrap().clone();
        let k = self.row.len();
        let mut next_row = Vec::with_capacity(self.row.len() + 1);
        next_row.push(self.iter.next()?);
        for n in 1..=k {
            next_row.push(next_row[n - 1].checked_add(&self.row[k - n])?);
        }
        self.row = next_row;
        Some(out)
    }
}

/// The entire triangle produced to generated the boustrophedon transform of a sequence
pub struct BoustrophedonTriangle<T> {
    iter: Box<dyn Iterator<Item = T>>,
    row: Vec<T>,
}

impl<T: CheckedAdd + Clone> BoustrophedonTriangle<T> {
    pub fn new<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            row: vec![iter.next().unwrap()],
            iter: Box::new(iter),
        }
    }
}

impl<T: CheckedAdd + Clone> Iterator for BoustrophedonTriangle<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut out = self.row.clone();
        let k = self.row.len();
        if k.is_even() {
            out.reverse();
        }
        let mut next_row = Vec::with_capacity(self.row.len() + 1);
        next_row.push(self.iter.next()?);
        for n in 1..=k {
            next_row.push(next_row[n - 1].checked_add(&self.row[k - n])?);
        }
        self.row = next_row;
        Some(out)
    }
}
