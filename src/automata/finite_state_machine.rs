use std::collections::HashMap;

/// The state transition function takes in a tape symbol then returns a state name.
pub struct State(Box<dyn Fn(&'static str) -> &'static str>);

/// The output function takes in a tape symbol and state name then returns a state name.
pub struct Output(Box<dyn Fn(&'static str, &'static str) -> &'static str>);

/// A deterministic finite state machine.
pub struct StateMachine {
    initial_state: &'static str,
    states: HashMap<&'static str, State>,
    output: Output,
}

impl StateMachine {
    pub fn new(
        initial_state: &'static str,
        states: HashMap<&'static str, State>,
        output: Output,
    ) -> Self {
        Self {
            initial_state,
            states,
            output,
        }
    }

    /// Run the automaton on an input.
    pub fn create_iter(&self, tape: Vec<&'static str>) -> StateMachineIter<'_> {
        StateMachineIter {
            tape,
            position: 0,
            current_state: self.initial_state,
            states: &self.states,
            output: &self.output,
        }
    }
}

// impl Automata for StateMachine {
//     type Item = &'static str;
//     fn run_automata(&self, input: Vec<Self::Item>) -> impl Iterator {
//         StateMachineIter {
//             tape: input,
//             position: 0,
//             current_state: self.initial_state,
//             states: &self.states,
//             output: &self.output,
//         }
//     }
// }

pub struct StateMachineIter<'a> {
    tape: Vec<&'static str>,
    position: usize,
    current_state: &'static str,
    states: &'a HashMap<&'static str, State>,
    output: &'a Output,
}

impl<'a> Iterator for StateMachineIter<'a> {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        // Automatically terminates at the end of the tape.
        let tape_symbol = *self.tape.get(self.position)?;
        let state = self
            .states
            .get(self.current_state)
            .expect("invalid state encountered");

        let out = Some(self.output.0(tape_symbol, self.current_state));
        self.current_state = state.0(tape_symbol);

        self.position += 1;
        out
    }
}

/// Create a HashMap relating the names of states to their transition functions.
#[macro_export]
macro_rules! fsm_states {
    ($(state $name_symbol: literal $($input:literal => $state:literal)+ )+) => {
        {
            let mut hmap = HashMap::new();
            $(
                hmap.insert(
                    $name_symbol,
                    State ( Box::new(|x: &'static str| -> &'static str {
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
            hmap
        }
    };
}

#[macro_export]
macro_rules! fsm_output{
    ( $($tape:literal, $state:literal => $out:literal)+ ) => {
        Output ( Box::new(|x: &'static str, y: &'static str | -> &'static str {
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
