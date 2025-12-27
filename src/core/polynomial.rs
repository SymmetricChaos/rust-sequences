use itertools::Itertools;
use num::{BigInt, CheckedAdd, CheckedMul, One, Signed, Zero};

use crate::core::{integer::Integers, natural::Naturals};

/// A polynomial evaluated at each natural number.
pub struct PolynomialNaturals<N> {
    coef: Vec<N>,
    ctr: Naturals<N>,
}

impl<N: One + Zero + CheckedAdd + CheckedMul + Clone> PolynomialNaturals<N> {
    pub fn new(coef: Vec<N>) -> Self {
        Self {
            coef,
            ctr: Naturals::new(),
        }
    }
}

impl PolynomialNaturals<BigInt> {
    pub fn new_big<T>(coef: Vec<T>) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            coef: coef.into_iter().map(|x| BigInt::from(x)).collect_vec(),
            ctr: Naturals::new_big(),
        }
    }
}

impl<N: One + Zero + CheckedAdd + CheckedMul + Clone> Iterator for PolynomialNaturals<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        let mut total = N::zero();
        for c in self.coef.iter().rev() {
            total = total.checked_mul(&self.ctr.next()?)?.checked_add(c)?;
        }
        Some(total)
    }
}

/// A polynomial evaluated at each integer.
pub struct PolynomialIntegers<N> {
    coef: Vec<N>,
    ctr: Integers<N>,
}

impl<N: One + Zero + CheckedAdd + CheckedMul + Signed + Clone> PolynomialIntegers<N> {
    pub fn new(coef: Vec<N>) -> Self {
        Self {
            coef,
            ctr: Integers::new(),
        }
    }
}

impl<N: One + Zero + CheckedAdd + CheckedMul + Signed + Clone> Iterator for PolynomialIntegers<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        let mut total = N::zero();

        for c in self.coef.iter().rev() {
            total = total.checked_mul(&self.ctr.next()?)?.checked_add(c)?;
        }
        Some(total)
    }
}
