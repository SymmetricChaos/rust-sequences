use crate::core::traits::Increment;

/// Hofstadter's H-sequence. A multiply recursive sequence starting with 0.
///
/// ```text
/// a(n) = n - a(a(a(n-1)))
/// 0, 1, 1, 2, 3, 4, 4, 5, 5, 6, 7, 7, 8, 9, 10...
/// ```
pub struct HofstadterH {
    terms: Vec<usize>,
    ctr: usize,
}

impl HofstadterH {
    pub fn new() -> Self {
        Self {
            terms: vec![0],
            ctr: 0,
        }
    }
}

impl Iterator for HofstadterH {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.terms.last()?.clone();

        self.ctr.incr()?;
        self.terms.push(self.ctr - self.terms[self.terms[out]]);

        Some(out)
    }
}

crate::check_sequences!(
    HofstadterH::new(), [0, 1, 1, 2, 3, 4, 4, 5, 5, 6, 7, 7, 8, 9, 10, 10, 11, 12, 13, 13, 14, 14, 15, 16, 17, 17, 18, 18, 19, 20, 20, 21, 22, 23, 23, 24, 24, 25, 26, 26, 27, 28];
);
