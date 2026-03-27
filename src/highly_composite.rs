use crate::{check_sequences, utils::divisibility::number_of_divisors};

/// The highly composite numbers, those which have mroe factors than any smaller number.
/// 1, 2, 4, 6, 12, 24, 36, 48, 60, 120...
pub struct HighlyComposite {
    ctr: u64,
    record_divisors: u64,
}

impl HighlyComposite {
    /// Only u64 output is supported.
    pub fn new() -> Self {
        Self {
            ctr: 0,
            record_divisors: 0,
        }
    }
}

impl Iterator for HighlyComposite {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.ctr = self.ctr.checked_add(1)?;
            let d = number_of_divisors(self.ctr);
            if d > self.record_divisors {
                self.record_divisors = d;
                return Some(self.ctr);
            }
        }
    }
}

check_sequences!(
    HighlyComposite::new(), [1, 2, 4, 6, 12, 24, 36, 48, 60, 120, 180, 240, 360];
);
