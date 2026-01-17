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

impl StateMachine {
    pub fn new(
        tape: Vec<&'static str>,
        initial_state: &'static str,
        states: Vec<(&'static str, State)>,
    ) -> Self {
        Self {
            tape,
            position: 0,
            current_state: initial_state,
            states: HashMap::from_iter(states),
        }
    }
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

#[macro_export]
macro_rules! fsm_state {
    ($name_symbol: literal; $($input:literal => $state:literal);+ $(;)?) => {
        ($name_symbol, State {
            func: Box::new(|x: &'static str| -> &'static str {
                match x {
                    $(
                        $input => $state,
                    )+
                    _ => panic!("symbol not handled"),
                }
            })
        })
    };
}

#[cfg(test)]
// #[ignore = "visualization"]
#[test]
fn busy_beaver() {
    let states = vec![
        fsm_state!(
            "Locked";
            "Coin" => "Unlocked";
            "Push" => "Locked";
        ),
        fsm_state!(
            "Unlocked";
            "Coin" => "Unlocked";
            "Push" => "Locked";
        ),
    ];

    let machine = StateMachine::new(
        vec!["Push", "Push", "Coin", "Push", "Coin", "Coin"],
        "Locked",
        states,
    );
    for (i, state) in machine.enumerate() {
        println!("{i:<2}  {}", state);
    }
}
