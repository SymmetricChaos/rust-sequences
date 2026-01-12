use std::collections::HashMap;

use crate::automata::components::{State, Tape};

pub struct PushdownAutomata {
    stack: Vec<char>,
    tape: Tape,
    states: HashMap<&'static str, State>,
    current_state: &'static str,
}

impl PushdownAutomata {
    pub fn new() -> Self {
        todo!()
    }
}
