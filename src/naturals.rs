pub struct Integers32 {
    ctr: u32,
}

impl Default for Integers32 {
    fn default() -> Self {
        Self { ctr: 0 }
    }
}

impl Integers32 {
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for Integers32 {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.ctr;
        self.ctr += 1;
        Some(out)
    }
}
