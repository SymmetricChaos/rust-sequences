use num::{BigInt, Integer, integer::binomial};

pub struct NaranyaTriangle<T> {
    row: T,
    idx: T,
}

impl<T: Integer + Clone> NaranyaTriangle<T> {
    pub fn new() -> Self {
        Self {
            row: T::one(),
            idx: T::one(),
        }
    }
}

impl NaranyaTriangle<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Integer + Clone> Iterator for NaranyaTriangle<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut out = Vec::new();
        loop {
            let a0 = binomial(self.row.clone() - T::one(), self.idx.clone() - T::one());
            let a1 = binomial(self.row.clone(), self.idx.clone() - T::one());
            out.push(a0 * a1 / self.idx.clone());
            self.idx = self.idx.clone() + T::one();
            if self.idx > self.row {
                self.row = self.row.clone() + T::one();
                self.idx = T::one();
                break;
            }
        }
        Some(out)
    }
}

crate::print_sequences!(
    NaranyaTriangle::new_big(), 5, "{:?}", "\n";
);
