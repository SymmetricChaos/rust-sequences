#[cfg(test)]
use crate::{
    automata::lindenmayer_system::Lindenmayer, automata::tag_machine::TagSystem, l_system,
    tag_system,
};

// Each term has a length that is equal to the matching term of the Padovan sequence
#[cfg(test)]
l_system!(
    padovan;
    'a' => "b"
    'b' => "c"
    'c' => "ab"
);

// Shortcut version of the Collatz sequence. Each term that is entire 'a's eventually generates a new all 'a's sequence with a length
// that is equal to (3n+1)/2 if the length is odd or n/2 if the length is even.
#[cfg(test)]
tag_system!(
    collatz;
    'a' => "bc"
    'b' => "a"
    'c' => "aaa"
);

crate::print_sequences!(
    Lindenmayer::new(padovan).create_iter("a"), 10, "{}", "\n";
    TagSystem::new(2, collatz, 'H').create_iter("aaa"), 30, "{}", "\n";
);
