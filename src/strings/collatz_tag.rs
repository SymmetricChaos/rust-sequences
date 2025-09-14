pub struct CollatzTag {
    string: String,
}

impl CollatzTag {
    pub fn new(n: usize) -> Self {
        let string = "a".repeat(n);

        Self { string }
    }
}

impl Iterator for CollatzTag {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.string.clone();

        let mut next_string = if self.string.len() <= 2 {
            String::new()
        } else {
            String::from(&self.string[2..])
        };

        match self.string.chars().next()? {
            'a' => next_string.push_str("bc"),
            'b' => next_string.push_str("a"),
            'c' => next_string.push_str("aaa"),
            _ => unreachable!(),
        }

        self.string = next_string;
        Some(out)
    }
}

crate::print_values!(
    CollatzTag::new(3), 0, 25;
    CollatzTag::new(4), 0, 7;
);
