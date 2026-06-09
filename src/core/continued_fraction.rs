#[cfg(feature = "big_int")]
use num::{BigInt, One, Zero};
use num::{CheckedAdd, CheckedMul, Integer, rational::Ratio};

/// Produce the convergents of a simple continued fraction given an integer sequence representing the partial denominators. The numerators are always 1.
pub struct SimpleContinuedFraction<T> {
    a0: T,
    b0: T,
    a1: T,
    b1: T,
    dens: Box<dyn Iterator<Item = T>>,
    ended: bool,
}

impl SimpleContinuedFraction<Number> {
    /// A simple continued fraction with denominators taken from an iterator.
    pub fn new<I>(mut d: I) -> Self
    where
        I: Iterator<Item = Number> + 'static,
    {
        Self {
            a0: 1,
            b0: 0,
            a1: d.next().unwrap(),
            b1: 1,
            dens: Box::new(d),
            ended: false,
        }
    }

    /// A simple continued fraction with some fixed starting denominators and then a periodic part.
    pub fn new_periodic(fixed: &[Number], periodic: &[Number]) -> Self {
        assert!(periodic.len() > 0);
        Self::new(Box::new(
            fixed
                .to_vec()
                .into_iter()
                .chain(periodic.to_vec().into_iter().cycle()),
        ))
    }

    /// A simple continued fraction with a finite number of terms.
    pub fn new_finite(dens: &[Number]) -> Self {
        assert!(dens.len() > 0);
        Self::new(Box::new(dens.to_vec().into_iter()))
    }
}

#[cfg(feature = "big_int")]
impl SimpleContinuedFraction<BigInt> {
    /// A simple continued fraction with denominators taken from an iterator.
    pub fn new_big<I>(mut d: I) -> Self
    where
        I: Iterator<Item = BigInt> + 'static,
    {
        Self {
            a0: BigInt::one(),
            b0: BigInt::zero(),
            a1: d.next().unwrap(),
            b1: BigInt::one(),
            dens: Box::new(d),
            ended: false,
        }
    }

    /// A simple continued fraction with some fixed starting denominators and then a periodic part.
    pub fn new_periodic_big(fixed: &[BigInt], periodic: &[BigInt]) -> Self {
        assert!(periodic.len() > 0);
        Self::new_big(Box::new(
            fixed
                .to_vec()
                .into_iter()
                .chain(periodic.to_vec().into_iter().cycle()),
        ))
    }

    /// A simple continued fraction with a finite number of terms.
    pub fn new_finite_big(dens: &[BigInt]) -> Self {
        assert!(dens.len() > 0);
        Self::new_big(Box::new(dens.to_vec().into_iter()))
    }
}

impl<T: CheckedAdd + CheckedMul + Clone + Integer> Iterator for SimpleContinuedFraction<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ended {
            return None;
        }
        let out = Ratio::new(self.a1.clone(), self.b1.clone());

        // Catch the last term of a finite sequence of denominators
        let d = match self.dens.next() {
            Some(d) => d,
            None => {
                self.ended = true;
                return Some(out);
            }
        };

        let a2 = d.checked_mul(&self.a1)?.checked_add(&self.a0)?;
        let b2 = d.checked_mul(&self.b1)?.checked_add(&self.b0)?;

        self.a0 = self.a1.clone();
        self.b0 = self.b1.clone();
        self.a1 = a2.clone();
        self.b1 = b2.clone();

        Some(out)
    }
}

use crate::Number;
#[cfg(test)]
use crate::core::{Naturals, traits::DigitSequence};
crate::print_sequences!(
    SimpleContinuedFraction::new_periodic(&[], &[1]).map(|q| q.digits(5).unwrap()), 10; // Converges on phi
    SimpleContinuedFraction::new_periodic(&[1], &[2]).map(|q| q.digits(5).unwrap()), 10; // Cnverges on sqrt(2)
    SimpleContinuedFraction::new_finite(&[3, 7, 15, 1, 292, 1]).map(|q| q.digits(10).unwrap()), 10; // Cnverges on pi, notice the jump in accuracy when the 292 term is reached
    SimpleContinuedFraction::new(Naturals::new()).map(|q| q.digits(10).unwrap()), 12; // converges on 0.697774657964007982
);
