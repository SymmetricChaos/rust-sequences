/// An tag system defined by:
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
    pub fn new<T>(init: String, deletion: usize, transition: T, halt: char) -> Self
    where
        T: Fn(char) -> Option<&'static str> + 'static,
    {
        assert!(deletion >= 1);
        Self {
            deletion,
            string: init,
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

#[macro_export]
macro_rules! tag_system {
    ($name:ident; $($a:literal => $b:literal);+ $(;)?) => {
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
    'a' => "ccbaH";
    'b' => "cca";
    'c' => "cc";
);

#[cfg(test)]
tag_system!(
    collatz_system;
    'a' => "bc";
    'b' => "a";
    'c' => "aaa";
);

crate::print_values!(
    TagSystem::new( String::from("baa"), 2, illustration_system, 'H'), 0, 10;
    TagSystem::new( String::from("aaa"), 2, collatz_system, 'H'), 0, 30;
);
