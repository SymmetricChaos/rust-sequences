use crate::utils::divisibility::aliquot_sum;

pub struct Abundant {
    n: u64,
}

impl Abundant {
    pub fn new() -> Self {
        Self { n: 11 }
    }
}

impl Iterator for Abundant {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.n += 1;
        while aliquot_sum(self.n).unwrap() <= self.n {
            self.n += 1;
        }
        Some(self.n)
    }
}

crate::check_sequences!(
    Abundant::new(), 0, 10, [12, 18, 20, 24, 30, 36, 40, 42, 48, 54];
);
