use crate::{Increment, core::parity::Odds};
use num::{BigInt, CheckedAdd, Integer};

/// Ludic numbers. Similar to the lucky numbers but terms are eliminated relative to the position of the number that eliminates them.
///
/// 1, 2, 3, 5, 7, 11, 13, 17, 23, 25, 29, 37, 41, 43, 47, 53, 61, 67...
pub struct Ludic<T> {
    ctr: usize,
    odds: Odds<T>,
    terms: Vec<[T; 2]>,
}

impl<T: CheckedAdd + Clone + Integer> Ludic<T> {
    pub fn new() -> Self {
        Self {
            ctr: 0,
            odds: Odds::new(),
            terms: Vec::new(),
        }
    }
}

impl Ludic<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: CheckedAdd + Clone + Integer> Iterator for Ludic<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // First term
        if self.ctr == 0 {
            self.ctr += 1;
            return self.odds.next();
        }

        // Second term
        if self.ctr == 1 {
            self.ctr += 1;
            self.terms.push([self.odds.next()?, T::zero()]);
            return Some(T::one() + T::one());
        }

        // Most recently added number will be output
        let out = self.terms.last()?[0].clone();

        'outer: loop {
            // Look at the next odd number.
            let n = self.odds.next()?;

            // In order (important!) step the counters for the known terms and stop if any of the counters reach a multiple of the term
            // Doing it this way ensures we do not double count anything and that we eliminate terms with the lower sequences first
            for [term, count] in self.terms.iter_mut() {
                count.incr()?;
                if (count.clone() % term.clone()).is_zero() {
                    continue 'outer;
                }
            }

            // Now that a new term has been determined it starts it own independent counter
            self.terms.push([n, T::zero()]);

            return Some(out);
        }
    }
}

crate::check_sequences!(
    Ludic::new_big(), [1, 2, 3, 5, 7, 11, 13, 17, 23, 25, 29, 37, 41, 43, 47, 53, 61, 67, 71, 77, 83, 89, 91, 97, 107, 115, 119, 121, 127, 131, 143, 149, 157, 161, 173, 175, 179, 181, 193, 209, 211, 221, 223, 227, 233, 235, 239, 247, 257, 265, 277, 283, 287, 301, 307, 313];
);
