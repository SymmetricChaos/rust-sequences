use std::collections::HashMap;

pub enum StackChange {
    Push(char),
    Pop,
    None,
}

// The state transition function takes in a tape symbol and a stack symbol. Based on these it returns the name of a State and optionally a symbol to push onto the stack.
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
    pub fn new(tape: Vec<char>, states: Vec<(&'static str, State)>, initial_stack: char) -> Self {
        Self {
            stack: vec![initial_stack],
            tape,
            position: 0,
            current_state: states[0].0,
            states: HashMap::from_iter(states),
        }
    }
}

impl Iterator for PushdownAutomata {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        let tape_symbol = self.tape.get(self.position)?.clone();
        let stack_symbol = self.stack.last()?.clone();
        let state = self.states.get(self.current_state)?;

        let (next_state, stack_change) = state.transition(tape_symbol, stack_symbol);

        self.current_state = next_state;
        self.position += 1;
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

#[macro_export]
macro_rules! pushdown_state {
    ($name_symbol: literal; $($t_input:literal, $s_input:literal => $state:literal, $change:expr);+ $(;)?) => {
        ($name_symbol, State {
            func: Box::new(|t: char, s:char| -> (&'static str, StackChange) {
                match (t,s) {
                    $(
                        ($t_input, $s_input) => ($state, $change),
                    )+
                    _ => panic!("symbol pair not handled"),
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
        pushdown_state!(
            "p";
            '0', 'Z' => "p", StackChange::Push('A');
            '0', 'A' => "p", StackChange::Push('A');
        ),
        pushdown_state!(
            "q";
            '1', 'A' => "q", StackChange::Pop;
        ),
    ];
}
