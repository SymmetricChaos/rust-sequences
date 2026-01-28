use std::collections::HashMap;

const BLANK: char = '\0';

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackChange {
    Push(char),
    Pop,
    Stay,
}

/// The state transition function takes in a tape symbol and a stack symbol. Based on these it returns the name of a State and optionally a symbol to push onto the stack.
pub struct State(Box<dyn Fn(char, char) -> (&'static str, StackChange)>);

/// A pushdown automata is effectively a finite state machine equipped with a stack.
pub struct PushdownAutomata {
    stack: Vec<char>,
    states: HashMap<&'static str, State>,
    current_state: &'static str,
    halting_states: Vec<&'static str>,
}

impl PushdownAutomata {
    pub fn new(
        states: HashMap<&'static str, State>,
        initial_state: &'static str,
        initial_stack: Option<Vec<char>>,
        halting_states: Vec<&'static str>,
    ) -> Self {
        if let Some(cs) = initial_stack {
            Self {
                stack: cs,
                current_state: initial_state,
                states,
                halting_states,
            }
        } else {
            Self {
                stack: vec![],
                current_state: initial_state,
                states,
                halting_states,
            }
        }
    }

    /// Take a tape of characters and run the automata on it.
    pub fn create_iter(&self, tape: Vec<char>) -> PushdownAutomataIter<'_> {
        PushdownAutomataIter {
            stack: self.stack.clone(),
            tape,
            position: 0,
            states: &self.states,
            current_state: self.current_state,
            halting_states: &self.halting_states,
        }
    }
}

pub struct PushdownAutomataIter<'a> {
    stack: Vec<char>,
    tape: Vec<char>,
    position: usize,
    states: &'a HashMap<&'static str, State>,
    current_state: &'static str,
    halting_states: &'a Vec<&'static str>,
}

impl<'a> Iterator for PushdownAutomataIter<'a> {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.halting_states.contains(&self.current_state) {
            return None;
        }

        let tape_symbol = *self.tape.get(self.position).unwrap_or(&BLANK);
        let stack_symbol = *self.stack.last().unwrap_or(&BLANK);
        let state = self.states.get(self.current_state)?;

        let (next_state, stack_change) = state.0(tape_symbol, stack_symbol);

        self.current_state = next_state;
        if self.position < self.tape.len() {
            self.position += 1;
        }

        match stack_change {
            StackChange::Push(c) => self.stack.push(c),
            StackChange::Pop => {
                self.stack.pop()?; // remove the top symbol
            }
            StackChange::Stay => (), // do nothing,
        }

        Some(next_state)
    }
}

/// Create a HashMap relating the names of states to their transition functions.
#[macro_export]
macro_rules! pushdown_states {
    ($(state $name_symbol: literal $($t_input:literal, $s_input:literal => $state:literal, $change:expr)+ )+) => {
        {
            use StackChange::*;
            let mut hmap = HashMap::new();
            $(
                hmap.insert(
                    $name_symbol,
                    State (Box::new(|t: char, s:char| -> (&'static str, StackChange) {
                            match (t,s) {
                                $(
                                    ($t_input, $s_input) => ($state, $change),
                                )+
                                _ => panic!("symbol pair (`{}`, `{}`) not handled in state {}", t,s, $name_symbol),
                            }
                        })
                    )
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
    use itertools::Itertools;

    // determine if the input consists of bitstring, then a 'c', then the reverse of the bitstring
    let states = pushdown_states![
        state "ADD"
            '1', '1' => "ADD", Push('1') // push a 1 or 0 whenever we find it
            '1', '0' => "ADD", Push('1')
            '1', '\0' => "ADD", Push('1')
            '0', '1' => "ADD", Push('0')
            '0', '0' => "ADD", Push('0')
            '0', '\0' => "ADD", Push('0')
            'c', '1' => "SUB", Stay // switch to SUB after finding a c
            'c', '0' => "SUB", Stay
            '\0', '1' => "NOT ACCEPT", Stay // do not accept if the tape runs out while in ADD
            '\0', '0' => "NOT ACCEPT", Stay
        state "SUB"
            '1', '1' => "SUB", Pop // if we find a 1 or 0 when we expect we pop them
            '0', '0' => "SUB", Pop
            '\0', '\0' => "ACCEPT", Stay // if the stack and tape are both empty we accept the input
            '1', '0' => "NOT ACCEPT", Stay // in all other cases we do not accept the string
            '0', '1' => "NOT ACCEPT", Stay
            'c', '1' => "NOT ACCEPT", Stay
            'c', '0' => "NOT ACCEPT", Stay
            '\0', '1' => "NOT ACCEPT", Stay
            '\0', '0' => "NOT ACCEPT", Stay
            '1', '\0' => "NOT ACCEPT", Stay
            '0', '\0' => "NOT ACCEPT", Stay
    ];
    let machine = PushdownAutomata::new(states, "ADD", None, vec!["ACCEPT", "NOT ACCEPT"]);

    let tapes = vec![
        vec!['1', '1', '0', '0', '1', '1'],
        vec!['1', '1', '0', 'c', '0', '1', '1'],
        vec!['1', '1', '0', 'c', '0', '0', '1'],
        vec!['1', '1', '0', 'c', '0', '1'],
    ];
    for tape in tapes {
        println!("\nCheck acceptance for the tape `{}`", tape.iter().join(""));
        for p in machine.create_iter(tape) {
            println!("{p}");
        }
    }
}
