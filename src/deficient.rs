use crate::utils::divisibility::aliquot_sum;

pub struct Deficient {
    n: u64,
}

impl Deficient {
    pub fn new() -> Self {
        Self { n: 0 }
    }
}

impl Iterator for Deficient {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.n += 1;
        while aliquot_sum(self.n).unwrap() >= self.n {
            self.n += 1;
        }
        Some(self.n)
    }
}

crate::check_sequences!(
    Deficient::new(), 0, 15, [1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 13, 14, 15, 16, 17];
);
