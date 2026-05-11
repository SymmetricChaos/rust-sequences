/// Create an elementary cellular automaton from a rule number using Wolfram's system.
pub struct ElementaryAutomaton {
    rule: [char; 8],
}

impl ElementaryAutomaton {
    pub fn new(rule: u8) -> Self {
        let mut arr: [char; 8] = ['0'; 8];
        for i in 0..8 {
            arr[i] = char::from(((rule >> (7 - i)) & 1) + 48); // bit twiddle to extract bits and get the '0' and '1' chars
        }
        Self { rule: arr }
    }

    /// Run the automaton on an input. At each stage the string is assumed to be padded with zeroes on each size.
    pub fn create_iter(&self, input: &str) -> ElementaryAutomatonIter<'_> {
        assert!(
            input.chars().all(|c| c == '0' || c == '1'),
            "input string can only contain 0s and 1s"
        );
        ElementaryAutomatonIter {
            string: input.to_string(),
            rule: &self.rule,
        }
    }
}

pub struct ElementaryAutomatonIter<'a> {
    string: String,
    rule: &'a [char; 8],
}

impl<'a> ElementaryAutomatonIter<'a> {
    fn apply_rule(&self, s: &str) -> char {
        match s {
            "111" => self.rule[0],
            "110" => self.rule[1],
            "101" => self.rule[2],
            "100" => self.rule[3],
            "011" => self.rule[4],
            "010" => self.rule[5],
            "001" => self.rule[6],
            "000" => self.rule[7],
            _ => unreachable!("only 3 char bit patterns will be provided"),
        }
    }
}

impl<'a> Iterator for ElementaryAutomatonIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.string.clone();
        self.string = format!("0{}0", self.string);
        let mut new = String::new();
        for i in 0..(self.string.len() - 2) {
            new.push(self.apply_rule(&self.string[i..(i + 3)]));
        }
        self.string = new;
        Some(out)
    }
}

crate::print_sequences!(
    ElementaryAutomaton::new(30).create_iter( "00000000000100000000000"), 12, "{}", "\n";
);
