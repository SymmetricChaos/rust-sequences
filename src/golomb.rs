pub struct Golomb {
    ctrs: Vec<u64>,
    idx: usize,
}

impl Golomb {
    pub fn new() -> Self {
        Self {
            ctrs: vec![1],
            idx: 0,
        }
    }
}

impl Iterator for Golomb {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.ctrs[self.idx];
        self.idx += 1;

        Some(out)
    }
}
