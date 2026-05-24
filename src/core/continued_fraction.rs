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

impl<T: CheckedAdd + CheckedMul + Clone + Integer + 'static> SimpleContinuedFraction<T> {
    /// A simple continued fraction with denominators taken from an iterator.
    pub fn new<I>(mut d: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            a0: T::one(),
            b0: T::zero(),
            a1: d.next().unwrap(),
            b1: T::one(),
            dens: Box::new(d),
            ended: false,
        }
    }

    /// A simple continued fraction with some fixed starting denominators and then a periodic part.
    pub fn new_periodic(fixed: &[T], periodic: &[T]) -> Self {
        assert!(fixed.len() > 0);
        assert!(periodic.len() > 0);
        Self::new(Box::new(
            fixed
                .to_vec()
                .into_iter()
                .chain(periodic.to_vec().into_iter().cycle()),
        ))
    }

    /// A simple continued fraction with a finite number of terms.
    pub fn new_finite(dens: &[T]) -> Self {
        assert!(dens.len() > 0);
        Self::new(Box::new(dens.to_vec().into_iter()))
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

#[cfg(test)]
use crate::core::{Naturals, traits::DigitSequence};
crate::print_sequences!(
    SimpleContinuedFraction::new_periodic(&[1], &[1]).map(|q| q.digits(5).unwrap()), 10; // Converges on phi
    SimpleContinuedFraction::new_periodic(&[1], &[2]).map(|q| q.digits(5).unwrap()), 10; // Cnverges on sqrt(2)
    SimpleContinuedFraction::new_finite(&[3, 7, 15, 1, 292, 1]).map(|q| q.digits(10).unwrap()), 10; // Cnverges on pi, notice the jump in accuracy when the 292 term is reached
    SimpleContinuedFraction::new(Naturals::new_big()).map(|q| q.digits(10).unwrap()), 12; // converges on 0.697774657964007982
);
