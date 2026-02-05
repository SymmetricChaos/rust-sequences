use crate::{core::prime::Primes, utils::divisibility::squarefree_kernel};
use num::{BigInt, CheckedAdd, CheckedMul, Integer, Zero};
use std::hash::Hash;

/// Squarefree numbers. Natural numbers that are not divisible twice by any natural number except one.
/// 1, 2, 3, 5, 6, 7, 10, 11, 13, 14...
pub struct Squarefree<T> {
    ctr: T,
    squares: Vec<T>,
    primes: Primes<T>,
}

impl<T: CheckedAdd + CheckedMul + Clone + Hash + Integer> Squarefree<T> {
    pub fn new() -> Self {
        let mut primes = Primes::new();
        primes.next();
        Self {
            ctr: T::zero(),
            squares: vec![T::one() + T::one() + T::one() + T::one()],
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

impl<T: CheckedAdd + CheckedMul + Clone + Hash + Integer> Iterator for Squarefree<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        'outer: loop {
            self.ctr = self.ctr.checked_add(&T::one())?;
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
/// 4, 8, 9, 12, 16, 18, 20, 24, 25, 27, 28...
pub struct Squareful<T> {
    ctr: T,
    squares: Vec<T>,
    primes: Primes<T>,
}

impl<T: CheckedAdd + CheckedMul + Clone + Hash + Integer> Squareful<T> {
    pub fn new() -> Self {
        let mut primes = Primes::new();
        primes.next();
        Self {
            ctr: T::zero(),
            squares: vec![T::one() + T::one() + T::one() + T::one()],
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

impl<T: Clone + CheckedAdd + CheckedMul + Integer> Iterator for Squareful<T>
where
    T: Hash,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        'outer: loop {
            self.ctr = self.ctr.checked_add(&T::one())?;
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

/// The squarefree kernels of the positive integers. The product of their unique prime divisors.
/// 1, 2, 3, 2, 5, 6, 7, 2, 3, 10, 11, 6, 13, 14, 15, 2, 17, 6, 19...
pub struct SquarefreeKernels {
    ctr: u64,
}

impl SquarefreeKernels {
    /// Only u64 currently supported
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for SquarefreeKernels {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr += 1;
        Some(squarefree_kernel(self.ctr))
    }
}

crate::check_sequences!(
    Squarefree::new_big(), 0, 20, [1, 2, 3, 5, 6, 7, 10, 11, 13, 14, 15, 17, 19, 21, 22, 23, 26, 29, 30, 31];
    SquarefreeKernels::new(), 0, 20, [1, 2, 3, 2, 5, 6, 7, 2, 3, 10, 11, 6, 13, 14, 15, 2, 17, 6, 19, 10];
    Squareful::new_big(), 0, 20, [4, 8, 9, 12, 16, 18, 20, 24, 25, 27, 28, 32, 36, 40, 44, 45, 48, 49, 50, 52];
);
