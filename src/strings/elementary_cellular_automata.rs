pub struct ElementaryCellularAutomaton {
    string: String,
    rule: [char; 8],
}

impl ElementaryCellularAutomaton {
    pub fn new(rule: u8, initial: &str) -> Self {
        assert!(
            initial.chars().all(|c| c == '0' || c == '1'),
            "initial string can only contain 0s and 1s"
        );
        let mut arr: [char; 8] = ['0'; 8];
        for i in 0..8 {
            arr[i as usize] = char::from(((rule >> (7 - i)) & 1) + 48);
        }
        println!("{:?}", arr);
        Self {
            string: initial.to_string(),
            rule: arr,
        }
    }

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

impl Iterator for ElementaryCellularAutomaton {
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

crate::print_values!(
    automata, formatter "{}", sep "\n";
    ElementaryCellularAutomaton::new(30, "000000000000010000000000000"), 0, 12;
);
