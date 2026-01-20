use num::{BigInt, Num};
use std::marker::PhantomData;

/// The sequence of look-and-say strings. Each is determiend by how the runs of numbers in the previous string might be said alound.
/// 1 (one one)
/// 11 (two one)
/// 21 (one two, one one)
/// 1211 (one one, one two, two one)
/// 111221 (three one, two two, one one)
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

/// The look-and-say sequence interpreted as numbers.
pub struct LookAndSay<T> {
    string: String,
    _phantom: PhantomData<T>,
}

impl<T> LookAndSay<T> {
    pub fn new() -> Self {
        Self {
            string: String::from("1"),
            _phantom: PhantomData,
        }
    }
}

impl LookAndSay<BigInt> {
    pub fn new_big() -> Self {
        Self {
            string: String::from("1"),
            _phantom: PhantomData,
        }
    }
}

impl<T: Num> Iterator for LookAndSay<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = T::from_str_radix(&self.string, 10).ok();

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

        out
    }
}

crate::print_values!(
    print_las, formatter "{}", sep ", ";
    LookAndSayString::new(), 0, 10;
    LookAndSay::new_big(), 0, 10;
    LookAndSay::<u32>::new(), 0, 10;
);
