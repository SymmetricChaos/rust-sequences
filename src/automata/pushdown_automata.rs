pub enum Move {
    Left,
    Right,
    Stay,
}

pub struct State {
    func: Box<dyn Fn(char) -> (char, Move, &'static str)>,
}

impl State {
    pub fn transition(&self, symbol: char) -> (char, Move, &'static str) {
        (self.func)(symbol)
    }
}

pub struct PushdownAutomata {
    stack: Vec<T>,
}

impl PushdownAutomata {
    pub fn new() -> Self {}
}
