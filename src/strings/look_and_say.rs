pub struct LookAndSayString {
    string: String,
}

impl LookAndSayString {
    pub fn new() -> Self {
        Self {
            string: String::from("1"),
        }
    }
}

impl Iterator for LookAndSayString {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.string.clone();

        let mut cur = '#'; // start with a char that does not occur
        let mut ctr = 0;
        let mut new = String::new();
        for c in self.string.chars() {
            if c != cur {
                if ctr != 0 {
                    new.push_str(&format!("{ctr}{cur}"));
                    ctr = 1;
                    cur = c;
                } else {
                    ctr = 1;
                    cur = c;
                }
            } else {
                ctr += 1;
            }
        }
        new.push_str(&format!("{ctr}{cur}"));
        self.string = new;

        Some(out)
    }
}

crate::print_values!(
    print_las, formatter "{}", sep "\n";
    LookAndSayString::new(), 0, 10;
);
