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
    states: HashMap<&'static str, Box<dyn Fn(char) -> (char, Move, &'static str)>>,
    current_state: &'static str,
}

impl TuringMachine {
    /// A new TuringMachine with a tape that has the
    pub fn new(
        tape: Vec<char>,
        blank: char,
        states: Vec<Box<dyn Fn(char) -> (char, Move, &'static str)>>,
        state_names: Vec<&'static str>,
    ) -> Self {
        if state_names.contains(&"HALT") {
            panic!("the HALT state is handled specially")
        }
        Self {
            tape: TuringTape::new(tape, 0, blank),
            states: HashMap::from_iter(state_names.iter().cloned().zip(states.into_iter())),
            current_state: state_names[0],
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
        let (symbol, direction, next_state) = self.states[self.current_state](cur_symbol);
        self.tape.write(symbol);
        self.tape.shift(direction);
        self.current_state = next_state;

        Some(out)
    }
}

#[macro_export]
macro_rules! turing_state {
    ($name:ident; $($a:literal => $symbol:literal, $movement:expr, $state:literal);+ $(;)?) => {
        fn $name(x: char) -> (char, Move, &'static str) {
            match x {
                $(
                    $a => ($symbol, $movement, $state),
                )+
                _ => panic!("symbol not handled"),
            }
        }
    };
}

#[cfg(test)]
#[ignore = "visualization"]
#[test]
fn test_tape() {
    let mut tape = TuringTape::new(vec!['1', '1', '1'], 0, '0');
    println!("{}", tape);
    for i in [
        Move::Left,
        Move::Left,
        Move::Right,
        Move::Right,
        Move::Right,
        Move::Right,
        Move::Right,
        Move::Right,
    ] {
        tape.shift(i);
        println!("{}", tape);
    }
}

#[cfg(test)]
#[ignore = "visualization"]
#[test]
fn busy_beaver() {
    turing_state!(
        state0;
        '0' => '1', Move::Right, "B";
        '1' => '1', Move::Left, "C";
    );

    turing_state!(
        state1;
        '0' => '1', Move::Left, "A";
        '1' => '1', Move::Right, "B";
    );

    turing_state!(
        state2;
        '0' => '1', Move::Left, "B";
        '1' => '1', Move::Right, "HALT";
    );

    let machine = TuringMachine::new(
        vec!['0'],
        '0',
        vec![Box::new(state0), Box::new(state1), Box::new(state2)],
        vec!["A", "B", "C"],
    );
    for tape in machine {
        println!("{}", tape);
    }
}
