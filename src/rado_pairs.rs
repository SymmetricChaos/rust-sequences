use crate::{Number, sorted_pairs::SortedPairsStrict};
use num::{BigInt, FromPrimitive, Integer, Zero, pow::Pow};

/// The pairs of numbers connected by an edge in the infinite Rado graph, with the lower term first. Alternatively every pair of numbers (a,b) such that the ath bit of b is 1.
///
/// ```text
/// (0, 1), (1, 2), (0, 3), (1, 3), (2, 4), (0, 5), (2, 5), (1, 6)...
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

    pub fn flattened() -> impl Iterator<Item = Number> {
        Self::new().flat_map(|x| [x.0, x.1])
    }
}

impl RadoPairs<BigInt> {
    pub fn new_big() -> Self {
        Self {
            pairs: SortedPairsStrict::new_big(),
        }
    }

    pub fn flattened_big() -> impl Iterator<Item = BigInt> {
        Self::new_big().flat_map(|x| [x.0, x.1])
    }
}

impl Iterator for RadoPairs<Number> {
    type Item = (Number, Number);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (a, b) = self.pairs.next()?;
            if (b >> a).is_odd() {
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
            let p2 = BigInt::from(2).pow(a.magnitude());
            if (&b / p2).is_odd() {
                return Some((a, b));
            }
        }
    }
}

crate::print_sequences!(
    RadoPairs::new(), 20, "{:?}", ", ";
    RadoPairs::new_big(), 20, "{:?}", ", ";
    RadoPairs::flattened(), 20;
);

crate::sample_sequences!(
    RadoPairs::new().map(|x| format!("{:?}",x));
);
