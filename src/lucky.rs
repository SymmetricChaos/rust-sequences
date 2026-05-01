use crate::{
    core::{parity::Odds, traits::Increment},
    transforms::complement::Complement,
};
use num::{BigInt, CheckedAdd, Integer};

/// The lucky numbers of number theory. They are produced using an algorithm similar to the Sieve of Eratosthenes, which produces the primes.
///
/// 1, 3, 7, 9, 13, 15, 21, 25, 31, 33...
pub struct Lucky<T> {
    ctr: T,
    odds: Odds<T>,
    terms: Vec<[T; 2]>,
}

impl<T: CheckedAdd + Clone + Integer> Lucky<T> {
    pub fn new() -> Self {
        Self {
            ctr: T::one(),
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
        if self.terms.is_empty() {
            let out = self.odds.next();
            self.terms.push([self.odds.next()?, T::one() + T::one()]);
            self.ctr.incr()?;
            return out;
        }

        // Most recently added number will be output
        let out = self.terms.last().unwrap()[0].clone();

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

            // Now that a new term has been determined step the overall counter
            // That gives us the value to start the new counter paired with the new term
            self.ctr.incr()?;
            self.terms.push([n, self.ctr.clone()]);

            return Some(out);
        }
    }
}

/// The unlucky numbers.
///
/// 2, 4, 5, 6, 8, 10, 11, 12, 14, 16, 17, 18, 19, 20, 22...
pub struct Unlucky<T>(Complement<T>);

impl<T: CheckedAdd + Clone + Integer + 'static> Unlucky<T> {
    pub fn new() -> Self {
        Self(Complement::new(Lucky::new(), T::one()))
    }
}

impl Unlucky<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: CheckedAdd + Clone + Integer> Iterator for Unlucky<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

crate::check_sequences!(
    Lucky::<i32>::new(), [1, 3, 7, 9, 13, 15, 21, 25, 31, 33, 37, 43, 49, 51, 63, 67, 69, 73, 75, 79, 87, 93, 99, 105, 111, 115, 127, 129, 133, 135, 141, 151, 159, 163, 169, 171, 189, 193, 195, 201, 205, 211, 219, 223, 231, 235, 237, 241, 259];
    Unlucky::<i32>::new(), [2, 4, 5, 6, 8, 10, 11, 12, 14, 16, 17, 18, 19, 20, 22, 23, 24, 26, 27, 28, 29, 30, 32, 34, 35, 36, 38, 39, 40, 41, 42, 44, 45, 46, 47, 48, 50, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 64, 65, 66, 68, 70, 71, 72, 74, 76, 77, 78, 80, 81, 82, 83, 84, 85, 86, 88, 89, 90, 91, 92];
);
