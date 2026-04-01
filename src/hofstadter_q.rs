pub struct HofstadterQ {
    terms: Vec<usize>,
    ctr: usize,
}

impl HofstadterQ {
    /// Only usize output is supported due to recursive structure.
    pub fn new() -> Self {
        Self {
            terms: vec![1, 1],
            ctr: 1,
        }
    }
}

impl Iterator for HofstadterQ {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr = self.ctr.checked_add(1)?;
        let out = self.terms[self.ctr - 2];

        let n = self.ctr;

        let a = self.terms[n - self.terms[n - 1]];
        let b = self.terms[n - self.terms[n - 2]];

        self.terms.push(a.checked_add(b)?);

        Some(out)
    }
}

crate::check_sequences!(
    HofstadterQ::new(), [1, 1, 2, 3, 3, 4, 5, 5, 6, 6, 6, 8, 8, 8, 10, 9, 10, 11, 11, 12, 12, 12, 12, 16];
);
