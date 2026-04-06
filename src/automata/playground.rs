#[cfg(test)]
mod tests {
    #[cfg(test)]
    use crate::{automata::markov_algorithm::Markov, markov_pairs};
    use crate::{
        automata::{lindenmayer_system::Lindenmayer, tag_machine::TagSystem},
        l_system, tag_system,
    };

    // Each term has a length that is equal to the matching term of the Padovan sequence
    l_system!(
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
            automata::turing_machine::{Move, Tape, TuringMachine},
            turing_states,
        };

        let bb_states = turing_states!(
            state "A"
                '0' => '1', Move::Right, "B"
                '1' => '1', Move::Left, "C"
            state "B"
                '0' => '1', Move::Left, "A"
                '1' => '1', Move::Right, "B"
            state "C"
                '0' => '1', Move::Left, "B"
                '1' => '1', Move::Right, "HALT"
        );

        let machine = TuringMachine::new("A", bb_states);

        let tape = Tape::new(vec!['0', '0', '0', '0', '0', '0'], 3, '0');

        for (i, state) in machine.create_iter(tape).enumerate() {
            println!("{i:<2}  {:<5} {}", state.0, state.1);
        }
    }
}
