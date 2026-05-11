/// A Markov string rewrite system. Given a string and ordered set of rules the first rule found to be applicable is applied once in the first applicable position at each step.
pub struct Markov {
    patterns: Vec<(String, String)>,
}

impl Markov {
    pub fn new(patterns: &[(String, String)]) -> Self {
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
    patterns: &'a Vec<(String, String)>,
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

/// Create a set of Markov rewrite rules.
///
/// ```
/// let pairs = markov_pairs!(
///     "I0" => "0II"
///     "1" => "0I"
///     "0" => ""
/// );
/// ```
#[macro_export]
macro_rules! markov_pairs {
    ($($a:literal => $b:literal)+) => {
        &[
            $(
                ($a.to_string(), $b.to_string()),
            )+
        ]
    };
}
