use std::collections::{HashMap, VecDeque};

pub enum Move {
    Left,
    Right,
    Stay,
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

pub struct TuringTape {
    tape: VecDeque<char>,
    position: usize,
    blank: char,
}

impl TuringTape {
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

pub struct TuringMachine {
    tape: TuringTape,
    position: usize,
    states: HashMap<&'static str, Box<dyn Fn(char) -> (char, Move, &'static str)>>,
    current_state: &'static str,
}

impl TuringMachine {
    pub fn new() -> Self {
        todo!()
    }
}

#[cfg(test)]
turing_state!(
    state0;
    '0' => '1', Move::Right, "B";
    '1' => '1', Move::Left, "C";
);

#[cfg(test)]
turing_state!(
    state1;
    '0' => '1', Move::Left, "A";
    '1' => '1', Move::Right, "B";
);

#[cfg(test)]
turing_state!(
    state2;
    '0' => '1', Move::Left, "B";
    '1' => '1', Move::Right, "B";
);
