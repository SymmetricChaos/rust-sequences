pub mod automata;
pub mod elementary_cellular_automata;
pub mod finite_state_machine;
pub mod lindenmayer_system;
pub mod markov_algorithm;
pub mod pushdown_automata;
pub mod tag_machine;
pub mod turing_machine;

pub trait Automata {
    type Input;
    type Output;

    /// Accept an input Vec and return an Iterator of outputs for each step as it run.
    fn create_automata(&self, input: Vec<Self::Input>) -> impl Iterator<Item = Self::Output>;
}
