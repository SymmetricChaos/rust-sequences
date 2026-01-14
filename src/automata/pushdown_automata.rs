use crate::automata::components::*;
use std::collections::HashMap;

pub struct State {
    pub func: Box<dyn Fn(char, char) -> &'static str>,
}

impl State {
    pub fn transition(&self, tape_symbol: char, stack_symbol: char) -> &'static str {
        (self.func)(tape_symbol, stack_symbol)
    }
}

pub struct PushdownAutomata {
    stack: Vec<char>,
    tape: Tape,
    states: HashMap<&'static str, State>,
    current_state: &'static str,
}

impl PushdownAutomata {
    pub fn new(initial_tape: Vec<char>, states: Vec<(&'static str, State)>) -> Self {
        todo!()
    }
}

impl Iterator for PushdownAutomata {
    type Item = Tape;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[macro_export]
macro_rules! pushdown_state {
    ($name_symbol: literal; $($t_input:literal, $s_input:literal  => $state:literal);+ $(;)?) => {
        ($name_symbol, State {
            func: Box::new(|t: char, s:char| -> (&'static str) {
                match (t,s) {
                    $(
                        ($t_input, $s_input) => $state,
                    )+
                    _ => panic!("symbol pair not handled"),
                }
            })
        })
    };
}
