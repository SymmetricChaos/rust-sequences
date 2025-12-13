use num::{BigInt, CheckedAdd, One, Zero};

use crate::{sorted_pairs::SortedPairs, utils::bit_predicate::bit_predicate};

/// The ordered pairs of numbers connected by an edge in the infinite Rado graph. Alternatively every pair of numbers (a,b) such that the ath digit of b is 1.
pub struct RadoPairs<T> {
    pairs: SortedPairs<T>,
}

impl<T: CheckedAdd + Clone + One + PartialOrd + Zero> RadoPairs<T> {
    pub fn new() -> Self {
        Self {
            pairs: SortedPairs::new(),
        }
    }
}

macro_rules! impl_rado_pairs {
    ($t: ty) => {
        impl Iterator for RadoPairs<$t> {
            type Item = ($t, $t);

            fn next(&mut self) -> Option<Self::Item> {
                loop {
                    let (a, b) = self.pairs.next()?;
                    if bit_predicate(a, b) {
                        return Some((a, b));
                    }
                }
            }
        }
    };
}

impl_rado_pairs!(u8);
impl_rado_pairs!(u16);
impl_rado_pairs!(u32);

impl Iterator for RadoPairs<BigInt> {
    type Item = (BigInt, BigInt);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (a, b) = self.pairs.next()?;
            let mut t = b.clone();
            for _ in num::iter::range(BigInt::one(), a.clone()) {
                t = t / 2;
            }
            if (t % BigInt::from(2)).is_one() {
                return Some((a, b));
            }
        }
    }
}

crate::print_values!(
    pairs, formatter "{:?}", sep ", ";
    RadoPairs::<u8>::new(), 0, 20;
);
