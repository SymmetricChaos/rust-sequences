use crate::{Increment, core::parity::Odds};
use num::{BigInt, CheckedAdd, Integer};

/// The lucky numbers of number theory. They are produced using an algorithm similar to how the Sieve of Eratosthenes produces the primes. They are often compared to the primes.
///
/// 1, 3, 7, 9, 13, 15, 21, 25, 31, 33...
pub struct Lucky<T> {
    ctr: T,
    counts: Vec<T>,
    odds: Odds<T>,
    terms: Vec<T>,
}

impl<T: CheckedAdd + Clone + Integer> Lucky<T> {
    pub fn new() -> Self {
        Self {
            ctr: T::one(),
            counts: Vec::new(),
            odds: Odds::new(),
            terms: Vec::new(),
        }
    }
}

impl Lucky<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: CheckedAdd + Clone + Integer> Iterator for Lucky<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // Handle first output
        if self.counts.is_empty() {
            let out = self.odds.next();
            self.terms.push(self.odds.next()?);
            self.counts.push(T::one() + T::one());
            self.ctr.incr()?;
            return out;
        }

        // Most recently added number will be output
        let out = self.terms.last().cloned();

        'outer: loop {
            // Look at the next odd number.
            let n = self.odds.next()?;

            // In order (important!) step the counters for the known terms and stop if any of the counters reach a multiple of the term
            // Doing it this way ensures we do not double count anything and that we eliminate terms with the lower sequences first
            for (term, count) in self.terms.iter().zip(self.counts.iter_mut()) {
                count.incr()?;
                if (count.clone() % term.clone()).is_zero() {
                    continue 'outer;
                }
            }

            // Now that a new term has been determined step the overall counter
            // That gives us the value to start the new counter paired with the new term
            self.ctr.incr()?;
            self.counts.push(self.ctr.clone());
            self.terms.push(n);

            return out;
        }
    }
}

crate::check_sequences!(
    Lucky::<i32>::new(), [1, 3, 7, 9, 13, 15, 21, 25, 31, 33, 37, 43, 49, 51, 63, 67, 69, 73, 75, 79, 87, 93, 99, 105, 111, 115, 127, 129, 133, 135, 141, 151, 159, 163, 169, 171, 189, 193, 195, 201, 205, 211, 219, 223, 231, 235, 237, 241, 259];
);
