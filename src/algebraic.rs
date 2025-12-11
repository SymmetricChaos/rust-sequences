use itertools::Itertools;

use crate::{core::polynomial::polynomial_string_signed, partition::PartitionsN};

/// All integer valued polynomials as ordered by Cantor.
pub struct Algebraic {
    partitions: PartitionsN,
    height: usize,
}

impl Algebraic {
    pub fn new() -> Self {
        Self {
            partitions: PartitionsN::new(0),
            height: 0,
        }
    }
}

/// TODO: needs to include negatives and permutations
impl Iterator for Algebraic {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(arr) = self.partitions.next() {
                if arr.len() <= 1 {
                    continue;
                } else {
                    let x = arr
                        .into_iter()
                        .map(|x| i32::try_from(x).unwrap())
                        .collect_vec();
                    return Some(polynomial_string_signed(&x, false));
                }
            } else {
                self.height += 1;
                self.partitions = PartitionsN::new(self.height);
            }
        }
    }
}

crate::print_values!(
    print_arrays, formatter "{:?}", sep "\n";
    Algebraic::new(), 0, 10;
);
