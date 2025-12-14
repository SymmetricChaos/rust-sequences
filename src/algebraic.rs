use itertools::Itertools;

use crate::{compositions_weak::WeakCompositionsNK, utils::polynomial::Polynomial};

/// All integer valued polynomials as ordered by Cantor.
pub struct Algebraic {
    height: usize,
    width: usize,
    weak_comps: WeakCompositionsNK,
}

impl Algebraic {
    pub fn new() -> Self {
        Self {
            height: 1,
            width: 1,
            weak_comps: WeakCompositionsNK::new(1, 1),
        }
    }
}

/// TODO: needs to include negatives and needs to sort correctly by height
impl Iterator for Algebraic {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(comp) = self.weak_comps.next() {
                let p = Polynomial::new(&comp.iter().map(|x| *x as i64).collect_vec());
                return Some(p.to_string());
            } else {
                self.width += 1;
                if self.width >= self.height {
                    self.width = 1;
                    self.height += 1;
                }
                self.weak_comps = WeakCompositionsNK::new(self.height, self.width);
            }
        }
    }
}

crate::print_values!(
    print_arrays, formatter "{}", sep "\n";
    Algebraic::new(), 0, 20;
);
