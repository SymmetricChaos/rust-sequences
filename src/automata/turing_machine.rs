use itertools::Itertools;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

/// Movement on a one dimensional Turing machine tape
#[derive(Clone, Copy, PartialEq, Eq)]
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
            if position >= tape.len() {
                panic!("position must be within the starting values given")
            }
            Self {
                tape: VecDeque::from(tape),
                position,
                blank,
            }
        }
    }

    /// Read the current symbol
    pub fn read(&self) -> char {
        self.tape[self.position]
    }

    /// Write a symbol at the current positions
    pub fn write(&mut self, symbol: char) {
        self.tape[self.position] = symbol;
    }

    /// Shift left or right or remain in the same position. The tape is infinite so new blanks can be inserted when this occurs.
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

    /// Concatenate all the symbols on the tape into a String.
    pub fn tape_symbols(&self) -> String {
        self.tape.iter().join("")
    }

    /// Remove any blanks trailing to the left or right of the tape then shrink the capacity of the underlying VecDeque.
    pub fn shrink(&mut self) {
        // If there is only one symbol then end immediately
        if self.tape.len() == 1 {
            return;
        }

        // Pop from the back until reaching a nonblank or the position
        loop {
            if self.position == self.tape.len() - 1 {
                break;
            }
            let s = self.tape.pop_back().unwrap();
            if s != self.blank {
                self.tape.push_back(s);
                break;
            }
        }

        // Pop from the front until either the position is at zero or a nonblank is found
        loop {
            if self.position == 0 {
                break;
            }
            let s = self.tape.pop_front().unwrap();
            if s != self.blank {
                self.tape.push_front(s);
                break;
            } else {
                self.position -= 1;
            }
        }

        self.tape.shrink_to_fit();
    }
}

/// Indicate the current position with a dot above the symbols.
impl Display for Tape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = " ".repeat(self.position);
        s.push('.');
        s.push('\n');
        s.push_str(&self.tape_symbols());
        write!(f, "{}", s)
    }
}

pub struct State {
    pub func: Box<dyn Fn(char) -> (char, Move, &'static str)>,
}

impl State {
    pub fn transition(&self, symbol: char) -> (char, Move, &'static str) {
        (self.func)(symbol)
    }
}

/// A one dimension Turing machine.
pub struct TuringMachine {
    tape: Tape,
    states: HashMap<&'static str, State>,
    current_state: &'static str,
}

impl TuringMachine {
    /// A new TuringMachine defined by a Tape, the name of the initial state, and a map of named States.
    pub fn new(
        tape: Tape,
        initial_state: &'static str,
        states: HashMap<&'static str, State>,
    ) -> Self {
        Self {
            tape,
            current_state: initial_state,
            states,
        }
    }
}

impl Iterator for TuringMachine {
    type Item = Tape;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_state == "HALT" {
            return None;
        }

        let out = self.tape.clone();

        let cur_symbol = self.tape.read();
        let (symbol, direction, next_state) =
            self.states[self.current_state].transition(cur_symbol);
        self.tape.write(symbol);
        self.tape.shift(direction);

        self.current_state = next_state;

        Some(out)
    }
}

/// Create a HashMap relating the names of states to their transition functions.
#[macro_export]
macro_rules! turing_states {
    ($(state $name_symbol: literal $($input:literal => $symbol:literal, $movement:expr, $state:literal)+ )+) => {
        {
            let mut hmap = HashMap::new();
            $(
                hmap.insert(
                    $name_symbol,
                    State {
                        func: Box::new(|x: char| -> (char, Move, &'static str) {
                            match x {
                                $(
                                    $input => ($symbol, $movement, $state),
                                )+
                                _ => panic!("symbol not handled"),
                            }
                        })
                    }
                );

            )+
            hmap
        }
    };
}

#[cfg(test)]
#[ignore = "visualization"]
#[test]
fn busy_beaver() {
    let states = turing_states!(
        state "A"
            '0' => '1', Move::Right, "B"
            '1' => '1', Move::Left, "C"
        state "B"
            '0' => '1', Move::Left, "A"
            '1' => '1', Move::Right, "B"
        state "C"
            '0' => '1', Move::Left, "B"
            '1' => '1', Move::Right, "HALT"
    );

    let machine = TuringMachine::new(
        Tape::new(vec!['0', '0', '0', '0', '0', '0'], 3, '0'),
        "A",
        states,
    );

    for (i, tape) in machine.enumerate() {
        println!("{i:<2}  {}", tape.tape_symbols());
    }

    // for (i, mut tape) in machine.enumerate() {
    //     tape.shrink();
    //     println!("{}", tape);
    // }
}
