pub trait Automaton {
    type Input;
    type Output;
    /// Transition between states and produce an output
    fn transition(&mut self, input: Self::Input) -> Self::Output;
    /// Is the Automaton in an accepting state?
    fn is_accepting(&self) -> bool;
}
