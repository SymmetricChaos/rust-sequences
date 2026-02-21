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
            height: 2,
            width: 2,
            weak_comps: WeakCompositionsNK::new(2, 2),
        }
    }
}

/// TODO: needs to include negatives and needs to sort correctly by height
impl Iterator for Algebraic {
    type Item = Polynomial<i64>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(comp) = self.weak_comps.next() {
                let p = Polynomial::new(comp.iter().map(|x| *x as i64).collect_vec());
                if p.is_constant() {
                    continue;
                }
                // if p.cantor_height().unwrap() != self.height {
                //     // println!("{} {} SKIP", p.cantor_height().expect("failed height"), p);
                //     continue;
                // }
                // // println!("{} {}", p.cantor_height().expect("failed height"), p);
                return Some(p);
            } else {
                self.width += 1;
                if self.width > self.height {
                    self.width = 1;
                    self.height += 1;
                }
                // println!("{} {}", self.width, self.height);

                self.weak_comps = WeakCompositionsNK::new(self.height, self.width);
            }
        }
        todo!()
    }
}

crate::print_sequences!(
    print_arrays, formatter "{}", sep "\n";
    Algebraic::new().map(|x| x.to_string_descending()), 0, 30;
);
