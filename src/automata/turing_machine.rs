use super::components::*;
use itertools::Itertools;
use std::collections::HashMap;

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
    /// A new TuringMachine. The initial_tape, position, and blank define a TuringTape. The states and state_names
    pub fn new(
        initial_tape: Vec<char>,
        initial_position: usize,
        blank: char,
        states: Vec<(&'static str, State)>,
    ) -> Self {
        if states.iter().map(|s| s.0).contains(&"HALT") {
            panic!("the HALT state is handled specially and must not be supplied")
        }
        if initial_position >= initial_tape.len() {
            panic!("position must be within the starting values give")
        }
        Self {
            tape: Tape::new(initial_tape, initial_position, blank),
            current_state: states[0].0,
            states: HashMap::from_iter(states),
        }
    }
}

impl Iterator for TuringMachine {
    type Item = Tape;

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
        ($name_symbol, State {
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
