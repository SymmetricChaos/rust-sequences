/// All integer valued polynomials as ordered by Cantor.
pub struct Algebraic {
    height: usize,
}

impl Algebraic {
    pub fn new() -> Self {
        Self { height: 0 }
    }
}

/// TODO: needs to include negatives and permutations
impl Iterator for Algebraic {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

crate::print_values!(
    print_arrays, formatter "{:?}", sep "\n";
    Algebraic::new(), 0, 10;
);
