pub struct Markov {
    string: String,
    patterns: Vec<(&'static str, &'static str)>,
}

impl Markov {
    pub fn new(initial: &str, patterns: &[(&'static str, &'static str)]) -> Self {
        Self {
            string: initial.to_string(),
            patterns: patterns.to_vec(),
        }
    }
}

impl Iterator for Markov {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.string.clone();
        for (pat, rep) in self.patterns.iter() {
            if self.string.contains(pat) {
                self.string = self.string.replacen(pat, rep, 1);
                break;
            }
        }
        Some(out)
    }
}

#[macro_export]
macro_rules! markov_pairs {
    ($($a:literal => $b:literal),+ $(,)?) => {
        &[
            $(
                ($a, $b),
            )+
        ]
    };
}

crate::print_values!(
    print_markov, formatter "{}", sep "\n";
    Markov::new("101",
        markov_pairs!(
            "I0" => "0II",
            "1" => "0I",
            "0" => "",
        )
    ), 0, 9;
);
