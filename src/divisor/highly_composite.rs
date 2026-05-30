use crate::{
    Number, check_sequences, core::traits::Increment, utils::divisibility::number_of_divisors,
};

/// The highly composite numbers, positive integers which have more divisors than any smaller positive integers.
///
/// ```text
/// 1, 2, 4, 6, 12, 24, 36, 48, 60, 120...
/// ```
pub struct HighlyComposite {
    ctr: Number,
    record_divisors: Number,
}

impl HighlyComposite {
    pub fn new() -> Self {
        Self {
            ctr: 0,
            record_divisors: 0,
        }
    }
}

impl Iterator for HighlyComposite {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.ctr.incr()?;
            let d = number_of_divisors(self.ctr);
            if d > self.record_divisors {
                self.record_divisors = d;
                return Some(self.ctr);
            }
        }
    }
}

/// The largely composite numbers, positive integers which have at least as many divisors as any smaller positive integers.
///
/// ```text
/// 1, 2, 3, 4, 6, 8, 10, 12, 18, 20, 24...
/// ```
pub struct LargelyComposite {
    ctr: Number,
    record_divisors: Number,
}

impl LargelyComposite {
    pub fn new() -> Self {
        Self {
            ctr: 0,
            record_divisors: 0,
        }
    }
}

impl Iterator for LargelyComposite {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.ctr.incr()?;
            let d = number_of_divisors(self.ctr);
            if d >= self.record_divisors {
                self.record_divisors = d;
                return Some(self.ctr);
            }
        }
    }
}

check_sequences!(
    HighlyComposite::new(), [1, 2, 4, 6, 12, 24, 36, 48, 60, 120, 180, 240, 360, 720, 840, 1260, 1680, 2520, 5040, 7560, 10080, 15120, 20160, 25200, 27720, 45360, 50400, 55440, 83160, 110880, 166320, 221760, 277200, 332640, 498960, 554400, 665280, 720720, 1081080, 1441440, 2162160];
    LargelyComposite::new(), [1, 2, 3, 4, 6, 8, 10, 12, 18, 20, 24, 30, 36, 48, 60, 72, 84, 90, 96, 108, 120, 168, 180, 240, 336, 360, 420, 480, 504, 540, 600, 630, 660, 672, 720, 840, 1080, 1260, 1440, 1680, 2160, 2520, 3360, 3780, 3960, 4200, 4320, 4620, 4680, 5040, 7560, 9240];
);
