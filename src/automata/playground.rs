#[cfg(test)]
mod tests {
    use crate::automata::finite_state_machine::StateMachine;
    use crate::automata::pushdown::PushdownAutomaton;
    use crate::{automata::markov_algorithm::Markov, markov_pairs};
    use crate::{
        automata::{lindenmayer_system::Lindenmayer, tag_machine::TagSystem},
        l_system_rules, tag_system,
    };
    use crate::{fsm_output, fsm_states, pushdown_states};

    // Each term has a length that is equal to the matching term of the Padovan sequence
    l_system_rules!(
        padovan;
        'a' => "b"
        'b' => "c"
        'c' => "ab"
    );

    // Shortcut version of the Collatz sequence. Each term that is entire 'a's eventually generates a new all 'a's sequence with a length
    // that is equal to (3n+1)/2 if the length is odd or n/2 if the length is even.
    tag_system!(
        collatz;
        'a' => "bc"
        'b' => "a"
        'c' => "aaa"
    );

    crate::print_sequences!(
        Lindenmayer::new(padovan).create_iter("a"), 10, "{}", "\n";
        TagSystem::new(2, collatz, 'H').create_iter("aaa"), 30, "{}", "\n";
        Markov::new(
            markov_pairs!(
                "I0" => "0II"
                "1" => "0I"
                "0" => ""
            )
        ).create_iter("101"), 12, "{}", "\n";
    );

    #[test]
    fn busy_beaver() {
        use crate::{
            automata::turing_machine::{TuringMachine, TuringMove, TuringTape},
            turing_states,
        };

        let bb_states = turing_states!(
            state "A"
                '0' => '1', Right, "B"
                '1' => '1', Left, "C"
            state "B"
                '0' => '1', Left, "A"
                '1' => '1', Right, "B"
            state "C"
                '0' => '1', Left, "B"
                '1' => '1', Right, "HALT"
        );

        let machine = TuringMachine::new("A", bb_states);

        let tape = TuringTape::new(vec!['0', '0', '0', '0', '0', '0'], 3, '0');

        for (i, state) in machine.create_iter(tape).enumerate() {
            println!("{i:<2}  {:<5} {}", state.0, state.1);
        }
    }

    #[test]
    fn pushdown_bit_counter() {
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
        let machine = PushdownAutomaton::new(states, "ADD", None, vec!["ACCEPT", "NOT ACCEPT"]);

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

    #[test]
    fn fsm_turnstile() {
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
}
