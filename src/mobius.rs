use crate::core::primality_utils::prime_factorization;
use num::{BigInt, CheckedAdd, Integer, One, Signed, Zero};
use std::marker::PhantomData;

/// The Möbius function over the positive integers
pub struct Mobius<T> {
    n: u64,
    _phantom: PhantomData<T>,
}

impl<T: One + Zero + Signed> Mobius<T> {
    pub fn new() -> Self {
        Self {
            n: 0,
            _phantom: PhantomData,
        }
    }
}

impl Mobius<BigInt> {
    pub fn new_big() -> Self {
        Self {
            n: 0,
            _phantom: PhantomData,
        }
    }
}

impl<T: One + Zero + Signed> Iterator for Mobius<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.n = self.n.checked_add(1)?;

        if self.n == 1 {
            return Some(T::one());
        } else {
            let pf = prime_factorization(self.n);
            if pf.iter().map(|x| x.1).any(|m| m > 1) {
                return Some(T::zero());
            } else {
                if pf.len().is_even() {
                    return Some(T::one());
                } else {
                    return Some(-T::one());
                }
            }
        }
    }
}

/// The values of the Merterns function over the positive integers. Partial sums of the Mobius function, without the leading zero.
pub struct Mertens<T> {
    n: u64,
    sum: T,
}

impl<T: CheckedAdd + Clone + One + Signed + Zero> Mertens<T> {
    pub fn new() -> Self {
        Self {
            n: 0,
            sum: T::zero(),
        }
    }
}

impl Mertens<BigInt> {
    pub fn new_big() -> Self {
        Self {
            n: 0,
            sum: BigInt::zero(),
        }
    }
}

impl<T: CheckedAdd + Clone + One + Signed + Zero> Iterator for Mertens<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.n = self.n.checked_add(1)?;

        if self.n == 1 {
            self.sum = self.sum.checked_add(&T::one())?;
        } else {
            let pf = prime_factorization(self.n);
            if pf.iter().map(|x| x.1).any(|m| m > 1) {
                // add zero
            } else {
                if pf.len().is_even() {
                    self.sum = self.sum.checked_add(&T::one())?;
                } else {
                    self.sum = self.sum.checked_add(&-T::one())?;
                }
            }
        }

        Some(self.sum.clone())
    }
}

crate::check_sequences!(
    Mobius::<i32>::new(), 0, 20, [1, -1, -1, 0, -1, 1, -1, 0, 0, 1, -1, 0, -1, 1, 1, 0, -1, 0, -1, 0];
    Mertens::<i32>::new(), 0, 20, [1, 0, -1, -1, -2, -1, -2, -2, -2, -1, -2, -2, -3, -2, -1, -1, -2, -2, -3, -3];
);
