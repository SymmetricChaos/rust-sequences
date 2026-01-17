pub struct Markov {
    string: String,
    patterns: Vec<(&'static str, &'static str)>,
    halted: bool,
}

impl Markov {
    pub fn new(initial: &str, patterns: &[(&'static str, &'static str)]) -> Self {
        Self {
            string: initial.to_string(),
            patterns: patterns.to_vec(),
            halted: false,
        }
    }
}

impl Iterator for Markov {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.halted {
            return None;
        }
        let out = self.string.clone();
        for (pat, rep) in self.patterns.iter() {
            if self.string.contains(pat) {
                self.string = self.string.replacen(pat, rep, 1);
                break;
            }
        }
        if self.string == out {
            self.halted = true;
        }
        Some(out)
    }
}

/// Nicer way to make the transformations
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
        markov_pairs!( // Converts a number from binary to bijective-unary
            "I0" => "0II",
            "1" => "0I",
            "0" => "",
        )
    ), 0, 12;
);
