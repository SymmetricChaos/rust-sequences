use std::{collections::HashMap, ops::Index};

/// The state transition function takes in a tape symbol then returns a state name.
pub struct State(pub Box<dyn Fn(&'static str) -> &'static str>);

pub struct States(pub HashMap<&'static str, State>);

impl Index<&str> for States {
    type Output = State;

    fn index(&self, index: &str) -> &Self::Output {
        &self.0[index]
    }
}

/// The output function takes in a tape symbol and state name then returns a state name.
pub struct Output(pub Box<dyn Fn(&'static str, &'static str) -> &'static str>);

/// A deterministic finite state machine.
pub struct StateMachine {
    initial_state_name: &'static str,
    states: States,
    output: Output,
}

impl StateMachine {
    pub fn new(initial_state_name: &'static str, states: States, output: Output) -> Self {
        Self {
            initial_state_name,
            states,
            output,
        }
    }

    /// Run the automaton on an input.
    pub fn create_iter(&self, tape: Vec<&'static str>) -> StateMachineIter<'_> {
        StateMachineIter {
            tape,
            position: 0,
            current_state_name: self.initial_state_name,
            states: &self.states,
            output: &self.output,
        }
    }
}

pub struct StateMachineIter<'a> {
    tape: Vec<&'static str>,
    position: usize,
    current_state_name: &'static str,
    states: &'a States,
    output: &'a Output,
}

impl<'a> Iterator for StateMachineIter<'a> {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        // Automatically terminates at the end of the tape.
        let tape_symbol = *self.tape.get(self.position)?;
        let state = &self.states[self.current_state_name];

        let out = Some(self.output.0(tape_symbol, self.current_state_name));
        self.current_state_name = state.0(tape_symbol);

        self.position += 1;
        out
    }
}

/// Create a HashMap relating the names of states to their transition functions.
///
/// Example:
/// ```
/// let turnstile_funcs = fsm_states!(
///     state "Locked"
///         "Coin" => "Unlocked"
///         "Push" => "Locked"
///     state "Unlocked"
///         "Coin" => "Unlocked"
///         "Push" => "Locked"
/// );
/// ```
#[macro_export]
macro_rules! fsm_states {
    ($(state $name_symbol: literal $($input:literal => $state:literal)+ )+) => {
        {
            let mut hmap = std::collections::HashMap::new();
            $(
                hmap.insert(
                    $name_symbol,
                    crate::automata::finite_state_machine::State ( Box::new(|x: &'static str| -> &'static str {
                            match x {
                                $(
                                    $input => $state,
                                )+
                                _ => panic!("symbol not handled"),
                            }
                        })
                    )
                );

            )+
            crate::automata::finite_state_machine::States(hmap)
        }
    };
}

/// Create a functon relating an input and state to an output.
///
/// ```
/// let turnstile_outputs = fsm_output!(
///     "Coin", "Locked" => "Coin Accepted, Unlocking"
///     "Push", "Locked" => "No Entry Allowed, Insert Coin"
///     "Coin", "Unlocked" => "Coin Wasted, Still Unlocked"
///     "Push", "Unlocked" => "One Entry Allowed, Locking"
/// );
/// ```
#[macro_export]
macro_rules! fsm_output{
    ( $($tape:literal, $state:literal => $out:literal)+ ) => {
        crate::automata::finite_state_machine::Output ( Box::new(|x: &'static str, y: &'static str | -> &'static str {
                match (x,y) {
                    $(
                        ($tape, $state) => $out,
                    )+
                    _ => panic!("tape symbol and state pair not handled"),
                }
            })
        )
    };
}

#[cfg(test)]
#[ignore = "visualization"]
#[test]
fn turnstile() {
    let states = fsm_states!(
        state "Locked"
            "Coin" => "Unlocked"
            "Push" => "Locked"
        state "Unlocked"
            "Coin" => "Unlocked"
            "Push" => "Locked"
    );

    let output = fsm_output!(
        "Coin", "Locked" => "Coin Accepted, Unlocking"
        "Push", "Locked" => "No Entry Allowed, Insert Coin"
        "Coin", "Unlocked" => "Coin Wasted, Still Unlocked"
        "Push", "Unlocked" => "One Entry Allowed, Locking"
    );

    let machine = StateMachine::new("Locked", states, output);

    let tape = vec!["Push", "Push", "Coin", "Push", "Coin", "Coin"];

    for (i, out) in machine.create_iter(tape).enumerate() {
        println!("{i:<2}  {}", out);
    }
}
