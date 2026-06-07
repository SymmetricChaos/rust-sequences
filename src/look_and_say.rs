use num::{BigInt, Num};
use std::marker::PhantomData;

use crate::Number;

/// The sequence of look-and-say strings. Each is determiend by how the runs of numbers in the previous string might be said aloud.
///
/// ```text
/// 1, 11, 21, 1211, 111221, 312211, 13112221, 1113213211...
/// ```
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
///
/// ```text
/// 1, 11, 21, 1211, 111221, 312211, 13112221, 1113213211...
/// ```
pub struct LookAndSay<T> {
    string: LookAndSayString,
    _phantom: PhantomData<T>,
}

impl LookAndSay<Number> {
    pub fn new() -> Self {
        Self {
            string: LookAndSayString::new(),
            _phantom: PhantomData,
        }
    }
}

impl LookAndSay<BigInt> {
    pub fn new_big() -> Self {
        Self {
            string: LookAndSayString::new(),
            _phantom: PhantomData,
        }
    }
}

impl<T: Num> Iterator for LookAndSay<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        T::from_str_radix(&self.string.next()?, 10).ok()
    }
}

crate::check_sequences!(
    LookAndSayString::new(), ["1", "11", "21", "1211", "111221", "312211", "13112221", "1113213211", "31131211131221", "13211311123113112211", "11131221133112132113212221", "3113112221232112111312211312113211"];
    LookAndSay::new_big(), [1_i64, 11, 21, 1211, 111221, 312211, 13112221, 1113213211, 31131211131221];
);

crate::sample_sequences!(
    LookAndSayString::new();
    LookAndSay::new_big();
);
