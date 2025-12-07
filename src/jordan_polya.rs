use crate::core::factorial::Factorials;

/// The Jordan–Pólya numbers, those that can be completely factored into factorial numbers
pub struct JordanPolya {
    ctr: u64,
    factorials_vec: Vec<u64>,
    factorials: Factorials<u64>,
}

impl JordanPolya {
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
            if self.factorials_vec.last().unwrap() <= &self.ctr {
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
    JordanPolya::new(), 0, 20, [1, 2, 4, 6, 8, 12, 16, 24, 32, 36, 48, 64, 72, 96, 120, 128, 144, 192, 216, 240];
);
