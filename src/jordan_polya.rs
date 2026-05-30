use crate::factorial::Factorials;

/// The Jordan–Pólya numbers, those that can be completely factored into factorial numbers.
///
/// ```text
/// 1, 2, 4, 6, 8, 12, 16, 24, 32...
/// ```
pub struct JordanPolya {
    ctr: u64,
    factorials_vec: Vec<u64>,
    factorials: Factorials<u64>,
}

impl JordanPolya {
    /// Only u64 output is supported.
    pub fn new() -> Self {
        let mut factorials = Factorials::new();
        factorials.next();
        factorials.next();
        Self {
            ctr: 0,
            factorials_vec: vec![factorials.next().unwrap()],
            factorials,
        }
    }
}

impl Iterator for JordanPolya {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.ctr += 1;
            if self.factorials_vec.last().unwrap() < &self.ctr {
                self.factorials_vec.push(self.factorials.next()?);
            }
            let mut m = self.ctr;
            for f in self.factorials_vec.iter().rev() {
                while m % f == 0 {
                    m /= f;
                }
            }
            if m == 1 {
                return Some(self.ctr);
            }
        }
    }
}

crate::check_sequences!(
    JordanPolya::new(), [1, 2, 4, 6, 8, 12, 16, 24, 32, 36, 48, 64, 72, 96, 120, 128, 144, 192, 216, 240, 256, 288, 384, 432, 480, 512, 576, 720, 768, 864, 960, 1024, 1152, 1296, 1440, 1536, 1728, 1920, 2048, 2304, 2592, 2880, 3072, 3456, 3840, 4096, 4320, 4608, 5040, 5184, 5760];
);
