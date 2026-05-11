use itertools::Itertools;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
    ops::Index,
};

pub struct TuringState(pub Box<dyn Fn(char) -> (char, TuringMove, String)>);

pub struct TuringStates(pub HashMap<String, TuringState>);

impl Index<&str> for TuringStates {
    type Output = TuringState;

    fn index(&self, index: &str) -> &Self::Output {
        &self.0[index]
    }
}

/// Movement on a one dimensional Turing machine tape.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TuringMove {
    Left,
    Right,
    Stay,
}

/// A one dimensional Turing machine tape.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TuringTape {
    tape: VecDeque<char>,
    position: usize,
    blank: char,
}

impl TuringTape {
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

    /// Read the current symbol.
    fn read(&self) -> char {
        self.tape[self.position]
    }

    /// Write a symbol at the current positions.
    fn write(&mut self, symbol: char) {
        self.tape[self.position] = symbol;
    }

    /// Shift left or right or remain in the same position. The tape is infinite so new blanks can be inserted when this occurs.
    fn shift(&mut self, direction: TuringMove) {
        match direction {
            TuringMove::Left => {
                if self.position == 0 {
                    self.tape.push_front(self.blank);
                } else {
                    self.position -= 1
                }
            }
            TuringMove::Right => {
                if self.position == self.tape.len() - 1 {
                    self.tape.push_back(self.blank);
                    self.position += 1;
                } else {
                    self.position += 1;
                }
            }
            TuringMove::Stay => {
                // do nothing
            }
        }
    }

    /// Concatenate all the symbols on the tape into a String and place a dot to indicate the head position.
    pub fn indicate_tape(&self) -> String {
        let mut s = " ".repeat(self.position);
        s.push('.');
        s.push('\n');
        s.push_str(&self.tape.iter().join(""));
        s
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
impl Display for TuringTape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tape.iter().join(""))
    }
}

/// A one dimension Turing machine.
pub struct TuringMachine {
    states: TuringStates,
    initial_state_name: String,
}

impl TuringMachine {
    /// A new TuringMachine defined by the name of the initial state, and a TuringStateMap that connects a name to each TuringState.
    pub fn new<S: ToString>(initial_state_name: S, states: TuringStates) -> Self {
        Self {
            initial_state_name: initial_state_name.to_string(),
            states,
        }
    }

    /// Run the automaton on a provided TuringTape.
    pub fn create_iter(&self, tape: TuringTape) -> TuringMachineIter<'_> {
        TuringMachineIter {
            tape,
            states: &self.states,
            current_state_name: self.initial_state_name.clone(),
        }
    }
}

pub struct TuringMachineIter<'a> {
    tape: TuringTape,
    states: &'a TuringStates,
    current_state_name: String,
}

impl<'a> Iterator for TuringMachineIter<'a> {
    type Item = (String, TuringTape);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_state_name == "HALT" {
            return None;
        }

        let cur_symbol = self.tape.read();
        let (symbol, direction, next_state) = self.states[&self.current_state_name].0(cur_symbol);
        self.tape.write(symbol);
        self.tape.shift(direction);

        self.current_state_name = next_state;

        Some((self.current_state_name.to_string(), self.tape.clone()))
    }
}

/// Create a HashMap relating the names of states to their transition functions.
///
/// Example:
/// ```
/// let bb_states = turing_states!(
///     state "A"
///        '0' => '1', Move::Right, "B"
///        '1' => '1', Move::Left, "C"
///     state "B"
///        '0' => '1', Move::Left, "A"
///        '1' => '1', Move::Right, "B"
///     state "C"
///        '0' => '1', Move::Left, "B"
///        '1' => '1', Move::Right, "HALT"
///    );
///```
#[macro_export]
macro_rules! turing_states {
    ($(state $name_symbol: literal $($input:literal => $symbol:literal, $movement:expr, $state:literal)+ )+) => {
        {
            use crate::automata::turing_machine::TuringMove::*;
            let mut hmap = std::collections::HashMap::new();
            $(
                hmap.insert(
                    $name_symbol.to_string(),
                    crate::automata::turing_machine::TuringState (
                        Box::new(|x: char| -> (char, TuringMove, String) {
                            match x {
                                $(
                                    $input => ($symbol, $movement, $state.to_string()),
                                )+
                                _ => panic!("symbol not handled"),
                            }
                        })
                    )
                );

            )+
            crate::automata::turing_machine::TuringStates(hmap)
        }
    };
}
