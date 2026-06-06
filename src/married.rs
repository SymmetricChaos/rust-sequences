/// Hofstadter's married sequences aka the male-female sequences. Defined by two entanged recurrences:
///
/// ```text
/// a(n) = n - b(a(n-1)), a(0) = 1
/// b(n) = n - a(b(n-1)), b(0) = 0
///
/// a sequence (female)
/// 1, 1, 2, 2, 3, 3, 4, 5, 5, 6, 6, 7, 8, 8, 9, 9, 10, 11, 11, 12, 13...
///
/// b sequence (male)
/// 0, 0, 1, 2, 2, 3, 4, 4, 5, 6, 6, 7, 7, 8, 9, 9, 10, 11, 11, 12, 12...
/// ```
pub struct Married {
    ctr: usize,
    a: Vec<usize>,
    b: Vec<usize>,
}

impl Married {
    /// Returns both sequences simultaneously.
    pub fn new() -> Self {
        Self {
            ctr: 0,
            a: vec![1],
            b: vec![0],
        }
    }

    /// Returns only the female (a) sequence.
    pub fn female() -> impl Iterator<Item = usize> {
        Self::new().map(|x| x.0)
    }

    /// Returns only the male (b) sequence.
    pub fn male() -> impl Iterator<Item = usize> {
        Self::new().map(|x| x.1)
    }
}

impl Iterator for Married {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let out = (self.a[self.ctr], self.b[self.ctr]);

        self.b.push(self.ctr + 1 - self.a[self.b[self.ctr]]);
        self.a.push(self.ctr + 1 - self.b[self.a[self.ctr]]);

        self.ctr += 1;

        Some(out)
    }
}

crate::check_sequences!(
    Married::female(), [1, 1, 2, 2, 3, 3, 4, 5, 5, 6, 6, 7, 8, 8, 9, 9, 10, 11, 11, 12, 13, 13, 14, 14, 15, 16, 16, 17, 17, 18, 19, 19, 20, 21, 21, 22, 22, 23, 24, 24, 25, 25, 26, 27];
    Married::male(),   [0, 0, 1, 2, 2, 3, 4, 4, 5, 6, 6, 7, 7, 8, 9, 9, 10, 11, 11, 12, 12, 13, 14, 14, 15, 16, 16, 17, 17, 18, 19, 19, 20, 20, 21, 22, 22, 23, 24, 24, 25, 25, 26, 27];
);

crate::sample_sequences!(
    Married::female();
    Married::male();
);
