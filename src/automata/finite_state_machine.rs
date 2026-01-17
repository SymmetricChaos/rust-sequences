use std::collections::HashMap;

/// The state transition function takes in a tape symbol and return a State.
pub struct State {
    pub func: Box<dyn Fn(&'static str) -> &'static str>,
}

impl State {
    pub fn transition(&self, tape_symbol: &'static str) -> &'static str {
        (self.func)(tape_symbol)
    }
}

pub struct StateMachine {
    tape: Vec<&'static str>,
    position: usize,
    current_state: &'static str,
    states: HashMap<&'static str, State>,
}

impl Iterator for StateMachine {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        // Automatically terminates at the end of the tape.
        let tape_symbol = *self.tape.get(self.position)?;
        let state = self
            .states
            .get(self.current_state)
            .expect("invalid state encountered");

        self.current_state = state.transition(tape_symbol);

        self.position += 1;

        Some(self.current_state)
    }
}
