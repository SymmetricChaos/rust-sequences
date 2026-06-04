use crate::{Number, sorted_pairs::SortedPairsStrict, utils::bit_predicate::bit_predicate};
use num::{BigInt, Integer, Zero};

/// The ordered pairs of numbers connected by an edge in the infinite Rado graph. Alternatively every pair of numbers (a,b) such that the ath digit of b is 1.
///
/// ```text
/// (0,1), (1,2), (0,3), (1,3), (2,4), (0,5), (2,5)
/// ```
pub struct RadoPairs<T> {
    pairs: SortedPairsStrict<T>,
}

impl RadoPairs<Number> {
    pub fn new() -> Self {
        Self {
            pairs: SortedPairsStrict::new(),
        }
    }
}

impl RadoPairs<BigInt> {
    pub fn new_big() -> Self {
        Self {
            pairs: SortedPairsStrict::new_big(),
        }
    }
}

impl Iterator for RadoPairs<Number> {
    type Item = (Number, Number);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (a, b) = self.pairs.next()?;
            if bit_predicate(a as u32, b as u32) || bit_predicate(b as u32, a as u32) {
                return Some((a, b));
            }
        }
    }
}

impl Iterator for RadoPairs<BigInt> {
    type Item = (BigInt, BigInt);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (a, b) = self.pairs.next()?;
            let mut t = b.clone();
            for _ in num::iter::range(BigInt::zero(), a.clone()) {
                t = t / 2;
            }
            if t.is_odd() {
                return Some((a, b));
            }
        }
    }
}

crate::print_sequences!(
    RadoPairs::new(), 20, "{:?}", ", ";
    RadoPairs::new_big(), 20, "{:?}", ", ";
);
