/// An tag system defined by:
/// The deletion number
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

#[cfg(test)]
fn illustration_system(x: char) -> Option<&'static str> {
    match x {
        'a' => Some("ccbaH"),
        'b' => Some("cca"),
        'c' => Some("cc"),
        _ => None,
    }
}

#[cfg(test)]
fn collatz_system(x: char) -> Option<&'static str> {
    match x {
        'a' => Some("bc"),
        'b' => Some("a"),
        'c' => Some("aaa"),
        _ => None,
    }
}

crate::print_values!(
    TagSystem::new( String::from("baa"), 2, illustration_system, 'H'), 0, 10;
    TagSystem::new( String::from("aaa"), 2, collatz_system, 'H'), 0, 31;
);
