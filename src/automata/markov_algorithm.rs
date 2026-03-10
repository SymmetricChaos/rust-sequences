/// Apply a Markov algorithm to a string.
pub struct Markov {
    patterns: Vec<(&'static str, &'static str)>,
}

impl Markov {
    pub fn new(patterns: &[(&'static str, &'static str)]) -> Self {
        Self {
            patterns: patterns.to_vec(),
        }
    }

    /// Run the automaton on an input.
    pub fn create_iter(&self, initial_string: &str) -> MarkovIter<'_> {
        MarkovIter {
            string: initial_string.to_string(),
            patterns: &self.patterns,
            halted: false,
        }
    }
}

pub struct MarkovIter<'a> {
    string: String,
    patterns: &'a Vec<(&'static str, &'static str)>,
    halted: bool,
}

impl<'a> Iterator for MarkovIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.halted {
            return None;
        }
        let out = self.string.clone();
        for (pattern, replacement) in self.patterns.iter() {
            if self.string.contains(pattern) {
                self.string = self.string.replacen(pattern, replacement, 1);
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
    ($($a:literal => $b:literal)+) => {
        &[
            $(
                ($a, $b),
            )+
        ]
    };
}

crate::print_sequences!(
    // Converts a number from binary to bijective-unary
    Markov::new(
        markov_pairs!(
            "I0" => "0II"
            "1" => "0I"
            "0" => ""
        )
    ).create_iter("101"), 12, "{}", "\n";
);
