use std::hash::Hash;

use crate::{
    Number,
    core::{primes::Primes, traits::Increment},
    utils::divisibility::radical,
};
use num::{BigInt, CheckedAdd, CheckedMul, Integer, Zero};

/// Squarefree numbers. Natural numbers that are not divisible twice by any natural number except one.
///
/// ```text
/// 1, 2, 3, 5, 6, 7, 10, 11, 13, 14, 15, 17, 19, 21, 22, 23, 26, 29...
/// ```
pub struct Squarefree<T> {
    ctr: T,
    squares: Vec<T>,
    primes: Primes<T>,
}

impl Squarefree<Number> {
    pub fn new() -> Self {
        let mut primes = Primes::new();
        primes.next();
        Self {
            ctr: 0,
            squares: vec![4],
            primes,
        }
    }
}

impl Squarefree<BigInt> {
    pub fn new_big() -> Self {
        let mut primes = Primes::new_big();
        primes.next();
        Self {
            ctr: BigInt::zero(),
            squares: vec![BigInt::from(4)],
            primes,
        }
    }
}

impl<T: Clone + CheckedAdd + CheckedMul + Integer + Hash> Iterator for Squarefree<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        'outer: loop {
            self.ctr.incr()?;
            if &self.ctr >= self.squares.last().unwrap() {
                let n = self.primes.next().unwrap();
                self.squares.push(n.checked_mul(&n)?);
            }
            for square in self.squares.iter() {
                if self.ctr.is_multiple_of(square) {
                    continue 'outer;
                }
            }
            break;
        }
        Some(self.ctr.clone())
    }
}

/// Positive natural numbers that are divisible at least twice by at least one natural number other than one. The numbers that are not squarefree.
///
/// ```text
/// 4, 8, 9, 12, 16, 18, 20, 24, 25, 27, 28, 32, 36, 40, 44, 45, 48, 49...
/// ```
pub struct Squareful<T> {
    ctr: T,
    squares: Vec<T>,
    primes: Primes<T>,
}

impl Squareful<Number> {
    pub fn new() -> Self {
        let mut primes = Primes::new();
        primes.next();
        Self {
            ctr: 0,
            squares: vec![4],
            primes,
        }
    }
}

impl Squareful<BigInt> {
    pub fn new_big() -> Self {
        let mut primes = Primes::new_big();
        primes.next();
        Self {
            ctr: BigInt::zero(),
            squares: vec![BigInt::from(4)],
            primes,
        }
    }
}

impl<T: Clone + CheckedAdd + CheckedMul + Integer + Hash> Iterator for Squareful<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        'outer: loop {
            self.ctr.incr()?;
            if &self.ctr >= self.squares.last().unwrap() {
                let n = self.primes.next().unwrap();
                self.squares.push(n.checked_mul(&n)?);
            }
            for square in self.squares.iter() {
                if self.ctr.is_multiple_of(square) {
                    break 'outer;
                }
            }
        }
        Some(self.ctr.clone())
    }
}

/// The radical of each positive integer, the product of their unique prime divisors. Also known as the squarefree kernel.
///
/// 1, 2, 3, 2, 5, 6, 7, 2, 3, 10, 11, 6, 13, 14, 15, 2, 17, 6, 19, 10...
pub struct Radicals {
    ctr: Number,
}

impl Radicals {
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for Radicals {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.incr()?;
        Some(radical(self.ctr))
    }
}

crate::check_sequences!(
    Squarefree::new_big(), [1, 2, 3, 5, 6, 7, 10, 11, 13, 14, 15, 17, 19, 21, 22, 23, 26, 29, 30, 31, 33, 34, 35, 37, 38, 39, 41, 42, 43, 46, 47, 51, 53, 55, 57, 58, 59, 61, 62, 65, 66, 67, 69, 70, 71, 73, 74, 77, 78, 79, 82, 83, 85, 86, 87, 89, 91, 93, 94, 95, 97, 101, 102, 103, 105, 106, 107, 109, 110, 111, 113];
    Radicals::new(),       [1, 2, 3, 2, 5, 6, 7, 2, 3, 10, 11, 6, 13, 14, 15, 2, 17, 6, 19, 10, 21, 22, 23, 6, 5, 26, 3, 14, 29, 30, 31, 2, 33, 34, 35, 6, 37, 38, 39, 10, 41, 42, 43, 22, 15, 46, 47, 6, 7, 10, 51, 26, 53, 6, 55, 14, 57, 58, 59, 30, 61, 62, 21, 2, 65, 66, 67, 34, 69, 70, 71, 6, 73, 74, 15, 38, 77, 78];
    Squareful::new_big(),  [4, 8, 9, 12, 16, 18, 20, 24, 25, 27, 28, 32, 36, 40, 44, 45, 48, 49, 50, 52, 54, 56, 60, 63, 64, 68, 72, 75, 76, 80, 81, 84, 88, 90, 92, 96, 98, 99, 100, 104, 108, 112, 116, 117, 120, 121, 124, 125, 126, 128, 132, 135, 136, 140, 144, 147, 148, 150, 152, 153, 156, 160];
);

crate::sample_sequences!(
    Squarefree::new();
    Radicals::new();
    Squareful::new();
);
