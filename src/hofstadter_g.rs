use crate::core::traits::Increment;

/// Hofstadter's G-sequence. A multiply recursive sequence starting with 0.
///
/// ```text
/// a(n) = n - a(a(n-1))
/// 0, 1, 1, 2, 3, 3, 4, 4, 5, 6, 6, 7, 8, 8, 9...
/// ```
pub struct HofstadterG {
    terms: Vec<usize>,
    ctr: usize,
}

impl HofstadterG {
    pub fn new() -> Self {
        Self {
            terms: vec![0],
            ctr: 0,
        }
    }
}

impl Iterator for HofstadterG {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.terms.last()?.clone();

        self.ctr.incr()?;
        self.terms.push(self.ctr - self.terms[out]);

        Some(out)
    }
}

crate::check_sequences!(
    HofstadterG::new(), [0, 1, 1, 2, 3, 3, 4, 4, 5, 6, 6, 7, 8, 8, 9, 9, 10, 11, 11, 12, 12, 13, 14, 14, 15, 16, 16, 17, 17, 18, 19, 19, 20, 21, 21, 22, 22, 23, 24, 24, 25, 25, 26, 27];
);
