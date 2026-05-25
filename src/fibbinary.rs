use crate::core::traits::Increment;

/// The Fibbinary numbers, the natural numbers, starting at zero, which contain no consecutive 1s in their binary expansion.
///
/// ```text
/// 0, 1, 2, 4, 5, 8, 9, 10, 16, 17, 18...
/// ```
pub struct Fibbinary {
    ctr: u64,
}

impl Fibbinary {
    pub fn new() -> Self {
        Self { ctr: 0 }
    }

    pub fn is_fibbinary(mut n: u64) -> bool {
        let mut prev = n & 1;
        for _ in 0..64 {
            n >>= 1;
            if n & 1 == 1 {
                if prev == 1 {
                    return false;
                }
            }
            prev = n & 1;
        }
        true
    }
}

impl Iterator for Fibbinary {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if Self::is_fibbinary(self.ctr) {
                let out = self.ctr;
                self.ctr.incr()?;
                return Some(out);
            }
            self.ctr.incr()?;
        }
    }
}

crate::check_sequences!(
    Fibbinary::new(), [0, 1, 2, 4, 5, 8, 9, 10, 16, 17, 18, 20, 21, 32, 33, 34, 36, 37, 40, 41, 42, 64, 65, 66, 68, 69, 72, 73, 74, 80, 81, 82, 84, 85, 128, 129, 130, 132, 133, 136, 137, 138, 144, 145, 146, 148, 149, 160, 161, 162, 164, 165, 168, 169, 170, 256, 257, 258, 260, 261, 264];
);
