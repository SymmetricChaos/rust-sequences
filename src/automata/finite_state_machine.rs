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

/// The output function takes in a tape symbol and State
pub struct Output {
    pub func: Box<dyn Fn(&'static str, &'static str) -> &'static str>,
}

impl Output {
    pub fn determine(&self, tape_symbol: &'static str, state: &'static str) -> &'static str {
        (self.func)(tape_symbol, state)
    }
}

/// A deterministic finite state machine.
pub struct StateMachine {
    tape: Vec<&'static str>,
    position: usize,
    current_state: &'static str,
    states: HashMap<&'static str, State>,
    output: Output,
}

impl StateMachine {
    pub fn new(
        tape: Vec<&'static str>,
        initial_state: &'static str,
        states: Vec<(&'static str, State)>,
        output: Output,
    ) -> Self {
        Self {
            tape,
            position: 0,
            current_state: initial_state,
            states: HashMap::from_iter(states),
            output,
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

        let out = Some(self.output.determine(tape_symbol, self.current_state));
        self.current_state = state.transition(tape_symbol);

        self.position += 1;
        out
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

#[macro_export]
macro_rules! fsm_output{
    ( $($tape:literal, $state:literal => $out:literal);+ $(;)?) => {
        Output {
            func: Box::new(|x: &'static str, y: &'static str | -> &'static str {
                match (x,y) {
                    $(
                        ($tape, $state) => $out,
                    )+
                    _ => panic!("tape symbol and state pair not handled"),
                }
            })
        }
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
    let output = fsm_output!(
        "Coin", "Locked" => "Coin Accepted, Unlocking";
        "Push", "Locked" => "No Entry Allowed, Insert Coin";
        "Coin", "Unlocked" => "Coin Wasted";
        "Push", "Unlocked" => "One Entry Allowed, Locking";
    );

    let machine = StateMachine::new(
        vec!["Push", "Push", "Coin", "Push", "Coin", "Coin"],
        "Locked",
        states,
        output,
    );
    for (i, out) in machine.enumerate() {
        println!("{i:<2}  {}", out);
    }
}
