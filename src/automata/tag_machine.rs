/// A tag system defined by:
/// The deletion number (the number of symbols deleted from the left of the word)
/// A function from char to Option<&'static str>
///     Constant symbols should return None and variables Some
/// A halting character
pub struct TagSystem {
    deletion: usize,
    string: String,
    transition: Box<dyn Fn(char) -> Option<&'static str>>,
    halted: bool,
    halt: char,
}

impl TagSystem {
    /// Panics if deletion is less than two.
    pub fn new<T>(initial: &str, deletion: usize, transition: T, halt: char) -> Self
    where
        T: Fn(char) -> Option<&'static str> + 'static,
    {
        assert!(deletion > 1);
        Self {
            deletion,
            string: initial.to_string(),
            transition: Box::new(transition),
            halted: false,
            halt,
        }
    }
}

impl Iterator for TagSystem {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.halted {
            return None;
        }

        let out = self.string.clone();

        let mut next_string = if self.string.len() < self.deletion {
            self.halted = true;
            String::new()
        } else {
            String::from(&self.string[self.deletion..])
        };

        if let Some(c) = self.string.chars().next() {
            if c == self.halt {
                self.halted = true;
            }
            if let Some(s) = (self.transition)(c) {
                next_string.push_str(s);
            } else {
                next_string.push(c);
            }
        } else {
            return None;
        }

        self.string = next_string;
        Some(out)
    }
}

/// The halting state of a Cyclic Tag System is the empty string.
pub struct CyclicTagSystem {
    string: String,
    productions: Vec<String>,
    ctr: usize,
}

impl CyclicTagSystem {
    /// Pancis if the init or any of the productions contain letters other than '0' and '1'
    pub fn new<S: ToString>(init: S, productions: &[S]) -> Self {
        let s = init.to_string();

        let productions: Vec<String> = productions.iter().map(|p| p.to_string()).collect();
        for pro in productions.iter() {
            for c in pro.chars() {
                if c != '0' && c != '1' {
                    panic!("only 0 and 1 are allowed in the productions string")
                }
            }
        }

        Self {
            string: s,
            productions,
            ctr: 0,
        }
    }
}

impl Iterator for CyclicTagSystem {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.string.clone();

        if let Some(c) = self.string.chars().next() {
            match c {
                '0' => self.string = self.string[1..].to_owned(),
                '1' => {
                    self.string = self.string[1..].to_owned();
                    self.string.push_str(&self.productions[self.ctr]);
                }
                _ => unreachable!("symbols other than '0' and '1' should never occur"),
            }
        } else {
            return None;
        }
        self.ctr = (self.ctr + 1) % self.productions.len();

        Some(out)
    }
}

#[macro_export]
macro_rules! tag_system {
    ($name:ident; $($a:literal => $b:literal),+ $(,)?) => {
        fn $name(x: char) -> Option<&'static str> {
            match x {
                $(
                    $a => Some($b),
                )+
                _ => None,
            }
        }
    };
}

#[cfg(test)]
tag_system!(
    illustration_system;
    'a' => "ccbaH",
    'b' => "cca",
    'c' => "cc",
);

#[cfg(test)]
tag_system!(
    collatz_system;
    'a' => "bc",
    'b' => "a",
    'c' => "aaa",
);

crate::print_values!(
    TagSystem::new("baa", 2, illustration_system, 'H'), 0, 10;
    TagSystem::new("aaa", 2, collatz_system, 'H'), 0, 30;
    CyclicTagSystem::new("11001", &["010", "000", "1111"]), 0, 10;
);
