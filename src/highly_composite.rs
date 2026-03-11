use crate::{check_sequences, utils::divisibility::number_of_divisors};

pub struct HighlyComposite {
    ctr: u64,
    divisors: u64,
}

impl HighlyComposite {
    pub fn new() -> Self {
        Self {
            ctr: 0,
            divisors: 0,
        }
    }
}

impl Iterator for HighlyComposite {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.ctr = self.ctr.checked_add(1)?;
            let d = number_of_divisors(self.ctr);
            if d > self.divisors {
                self.divisors = d;
                return Some(self.ctr);
            }
        }
    }
}

check_sequences!(
    HighlyComposite::new(), [1, 2, 4, 6, 12, 24, 36, 48, 60, 120, 180, 240, 360];
);
