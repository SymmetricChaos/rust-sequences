/// Lagged Fibonacci Generator.
///
/// ```text
/// a = 123, b = 456, m = 789
/// 123, 456, 579, 246, 36, 282, 318, 600, 129, 729, 69, 9, 78, 87, 165...
/// ```
pub struct Lfg {
    a: u64,
    b: u64,
    m: u64,
}

impl Lfg {
    pub fn new(a: u32, b: u32, m: u32) -> Self {
        Self {
            a: a as u64,
            b: b as u64,
            m: m as u64,
        }
    }
}

impl Iterator for Lfg {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a;
        let t = (self.a + self.b) % self.m;
        self.a = self.b;
        self.b = t;
        Some(out)
    }
}

crate::sample_sequences!(
    Lfg::new(123, 456, 789);
);
