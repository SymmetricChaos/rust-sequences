#[cfg(test)]
use crate::{automata::lindenmayer_system::Lindenmayer, l_system};

// Each term has a length that is equal to the matching term of the Padovan sequence
#[cfg(test)]
l_system!(
    padovan;
    'a' => "b"
    'b' => "c"
    'c' => "ab"
);

crate::print_sequences!(
    Lindenmayer::new(padovan).create_iter("a"),10,"{}","\n";
);
