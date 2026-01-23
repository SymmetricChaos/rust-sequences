use std::collections::HashMap;

const BLANK: char = '\0';

pub enum StackChange {
    Push(char),
    Pop,
    None,
}

/// The state transition function takes in a tape symbol and a stack symbol. Based on these it returns the name of a State and optionally a symbol to push onto the stack.
pub struct State {
    pub func: Box<dyn Fn(char, char) -> (&'static str, StackChange)>,
}

impl State {
    pub fn transition(&self, tape_symbol: char, stack_symbol: char) -> (&'static str, StackChange) {
        (self.func)(tape_symbol, stack_symbol)
    }
}

pub struct PushdownAutomata {
    stack: Vec<char>,
    tape: Vec<char>,
    position: usize,
    states: HashMap<&'static str, State>,
    current_state: &'static str,
}

impl PushdownAutomata {
    pub fn new(
        tape: Vec<char>,
        states: HashMap<&'static str, State>,
        initial_state: &'static str,
        initial_stack_symbol: char,
    ) -> Self {
        Self {
            stack: vec![initial_stack_symbol],
            tape,
            position: 0,
            current_state: initial_state,
            states,
        }
    }
}

impl Iterator for PushdownAutomata {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_state == "HALT" {
            return None;
        }

        let tape_symbol = *self.tape.get(self.position).unwrap_or(&BLANK);
        let stack_symbol = *self.stack.last().unwrap_or(&BLANK);
        let state = self.states.get(self.current_state)?;

        let (next_state, stack_change) = state.transition(tape_symbol, stack_symbol);

        self.current_state = next_state;
        if self.position < self.tape.len() {
            self.position += 1;
        }

        match stack_change {
            StackChange::Push(c) => self.stack.push(c),
            StackChange::Pop => {
                self.stack.pop()?; // remove the top symbol
            }
            StackChange::None => (), // do nothing,
        }

        Some(next_state)
    }
}

/// Create a HashMap relating the names of states to their transition functions.
#[macro_export]
macro_rules! pushdown_states {
    ($(state $name_symbol: literal $($t_input:literal, $s_input:literal => $state:literal, $change:expr)+ )+) => {
        {
            let mut hmap = HashMap::new();
            $(
                hmap.insert(
                    $name_symbol,
                    State {
                        func: Box::new(|t: char, s:char| -> (&'static str, StackChange) {
                            match (t,s) {
                                $(
                                    ($t_input, $s_input) => ($state, $change),
                                )+
                                _ => panic!("symbol pair not handled"),
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
fn bit_counter() {
    let states = pushdown_states![
        state "p"
            '0', 'Z' => "p", StackChange::Push('A')
            '0', 'A' => "p", StackChange::Push('A')
        state "q"
            '1', 'A' => "q", StackChange::Pop
    ];
    let machine = PushdownAutomata::new(vec!['1', '1', '0', '1', '0', '1'], states, "p", 'z');
}
