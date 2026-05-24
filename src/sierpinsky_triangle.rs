///Sierpiński's triangle
///
///
pub struct SierpinskyTriangle {
    ctr: usize,
}

impl SierpinskyTriangle {
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for SierpinskyTriangle {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.ctr;
        let mut out = Vec::new();
        for k in 0..=n {
            if k & !n == 0 {
                out.push(1);
            } else {
                out.push(0);
            }
        }
        self.ctr += 1;
        Some(out)
    }
}

crate::print_sequences!(
    SierpinskyTriangle::new(), 5, "{:?}", "\n";
);

crate::check_sequences!(
    SierpinskyTriangle::new().flatten(), [1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1];
);
