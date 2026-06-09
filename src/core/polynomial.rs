use crate::{
    Number,
    core::{integer::Integers, natural::Naturals},
};
use itertools::Itertools;
use num::{BigInt, CheckedAdd, CheckedMul, Zero};

/// A polynomial with integer coefficients evaluated at each value of a given seqeunce.
pub struct Polynomial<T> {
    coefs: Vec<T>,
    points: Box<dyn Iterator<Item = T>>,
}

impl<T> Polynomial<T> {
    pub fn new<I>(coefs: Vec<T>, points: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            coefs,
            points: Box::new(points),
        }
    }
}

impl Polynomial<Number> {
    /// Evaluted at each natural number.
    pub fn at_naturals(coefs: Vec<Number>) -> Self {
        Self {
            coefs,
            points: Box::new(Naturals::new()),
        }
    }

    /// Evaluted at each integer.
    pub fn at_integers(coefs: Vec<Number>) -> Self {
        Self {
            coefs,
            points: Box::new(Integers::new()),
        }
    }
}

impl Polynomial<BigInt> {
    /// Evaluted at each natural number.
    pub fn at_naturals_big<G>(coefs: Vec<G>) -> Self
    where
        BigInt: From<G>,
    {
        Self {
            coefs: coefs.into_iter().map(|c| BigInt::from(c)).collect_vec(),
            points: Box::new(Naturals::new_big()),
        }
    }

    /// Evaluted at each integer.
    pub fn at_integers_big<G>(coefs: Vec<G>) -> Self
    where
        BigInt: From<G>,
    {
        Self {
            coefs: coefs.into_iter().map(|c| BigInt::from(c)).collect_vec(),
            points: Box::new(Integers::new_big()),
        }
    }
}

impl<T: Zero + CheckedAdd + CheckedMul + Clone> Iterator for Polynomial<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut total = T::zero();
        let x = self.points.next()?;
        for c in self.coefs.iter().rev() {
            total = total.checked_mul(&x)?.checked_add(c)?;
        }

        Some(total)
    }
}

crate::check_sequences!(
    Polynomial::at_naturals(vec![-6,1,2]), [-6, -3, 4, 15, 30, 49, 72, 99, 130, 165];
    Polynomial::at_integers(vec![1,-4,3]), [1, 0, 8, 5, 21, 16, 40, 33, 65, 56];
);
