use itertools::Itertools;
use std::{collections::VecDeque, fmt::Display};

pub enum Move {
    Left,
    Right,
    Stay,
}

/// A one dimensional Turing machine tape
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tape {
    pub tape: VecDeque<char>,
    pub position: usize,
    pub blank: char,
}

impl Tape {
    pub fn new(tape: Vec<char>, position: usize, blank: char) -> Self {
        if tape.is_empty() {
            Self {
                tape: VecDeque::from([blank]),
                position,
                blank,
            }
        } else {
            Self {
                tape: VecDeque::from(tape),
                position,
                blank,
            }
        }
    }

    pub fn read(&self) -> char {
        self.tape[self.position]
    }

    pub fn write(&mut self, symbol: char) {
        self.tape[self.position] = symbol;
    }

    pub fn shift(&mut self, direction: Move) {
        match direction {
            Move::Left => {
                if self.position == 0 {
                    self.tape.push_front(self.blank);
                } else {
                    self.position -= 1
                }
            }
            Move::Right => {
                if self.position == self.tape.len() - 1 {
                    self.tape.push_back(self.blank);
                    self.position += 1;
                } else {
                    self.position += 1;
                }
            }
            Move::Stay => {
                // do nothing
            }
        }
    }

    pub fn tape_symbols(&self) -> String {
        self.tape.iter().join("")
    }
}

impl Display for Tape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = " ".repeat(self.position);
        s.push('.');
        s.push('\n');
        s.push_str(&self.tape.iter().join(""));
        write!(f, "{}", s)
    }
}
