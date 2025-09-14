pub struct LindenmayerAlgae {
    string: String,
}

impl LindenmayerAlgae {
    pub fn new() -> Self {
        Self {
            string: String::from("a"),
        }
    }
}

impl Iterator for LindenmayerAlgae {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.string.clone();
        let mut next = String::with_capacity(self.string.len());
        for c in self.string.chars() {
            match c {
                'a' => next.push_str("ab"),
                'b' => next.push_str("a"),
                _ => unreachable!(),
            }
        }
        self.string = next;
        Some(out)
    }
}

crate::print_values!(
    LindenmayerAlgae::new(), 0, 7;
);
