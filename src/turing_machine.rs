use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

use itertools::Itertools;

pub enum Move {
    Left,
    Right,
    Stay,
}

pub struct TuringState {
    func: Box<dyn Fn(char) -> (char, Move, &'static str)>,
}

impl TuringState {
    pub fn transition(&self, symbol: char) -> (char, Move, &'static str) {
        (self.func)(symbol)
    }
}

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

impl Display for TuringTape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = " ".repeat(self.position);
        s.push('.');
        s.push('\n');
        s.push_str(&self.tape.iter().join(""));
        write!(f, "{}", s)
    }
}

pub struct TuringMachine {
    tape: TuringTape,
    states: HashMap<&'static str, TuringState>,
    current_state: &'static str,
}

impl TuringMachine {
    /// A new TuringMachine. The initial_tape, position, and blank define a TuringTape. The states and state_names
    pub fn new(
        initial_tape: Vec<char>,
        position: usize,
        blank: char,
        states: Vec<(&'static str, TuringState)>,
    ) -> Self {
        if states.iter().map(|s| s.0).contains(&"HALT") {
            panic!("the HALT state is handled specially and must not be supplied")
        }
        if position >= initial_tape.len() {
            panic!("position must be within the starting values give")
        }
        Self {
            tape: TuringTape::new(initial_tape, position, blank),
            current_state: states[0].0,
            states: HashMap::from_iter(states),
        }
    }
}

impl Iterator for TuringMachine {
    type Item = TuringTape;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.tape.clone();

        if self.current_state == "HALT" {
            return None;
        }

        let cur_symbol = self.tape.read();
        let (symbol, direction, next_state) =
            self.states[self.current_state].transition(cur_symbol);
        self.tape.write(symbol);
        self.tape.shift(direction);
        self.current_state = next_state;

        Some(out)
    }
}

#[macro_export]
macro_rules! turing_state {
    ($name_symbol: literal; $($input:literal => $symbol:literal, $movement:expr, $state:literal);+ $(;)?) => {
        ($name_symbol, TuringState {
            func: Box::new(|x: char| -> (char, Move, &'static str) {
                match x {
                    $(
                        $input => ($symbol, $movement, $state),
                    )+
                    _ => panic!("symbol not handled"),
                }
            })
        })
    };
}

#[cfg(test)]
#[ignore = "visualization"]
#[test]
fn busy_beaver() {
    let states = vec![
        turing_state!(
            "A";
            '0' => '1', Move::Right, "B";
            '1' => '1', Move::Left, "C";
        ),
        turing_state!(
            "B";
            '0' => '1', Move::Left, "A";
            '1' => '1', Move::Right, "B";
        ),
        turing_state!(
            "C";
            '0' => '1', Move::Left, "B";
            '1' => '1', Move::Right, "HALT";
        ),
    ];

    let machine = TuringMachine::new(vec!['0'], 0, '0', states);
    for (i, tape) in machine.enumerate() {
        println!("{i:<2}  {}", tape.tape_symbols());
    }
}
