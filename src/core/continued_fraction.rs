use num::{CheckedAdd, CheckedMul, Integer, rational::Ratio};

use crate::print_sequences;

// /// Produce the convergents of a continued fraction given integer sequences representing the partial numerators and partial denominators.
// pub struct ContinuedFraction<T> {
//     a0: T,
//     b0: T,
//     a1: T,
//     b1: T,
//     numerators: Box<dyn Iterator<Item = T>>,
//     denominators: Box<dyn Iterator<Item = T>>,
// }

// impl<T: CheckedAdd + CheckedMul + Clone + Integer + 'static> ContinuedFraction<T> {
//     pub fn new<N, D>(n: N, mut d: D) -> Self
//     where
//         N: Iterator<Item = T> + 'static,
//         D: Iterator<Item = T> + 'static,
//     {
//         Self {
//             a0: T::one(),
//             b0: T::zero(),
//             a1: d.next().unwrap(),
//             b1: T::one(),
//             numerators: Box::new(n),
//             denominators: Box::new(d),
//         }
//     }

//     pub fn new_periodic(
//         fixed_n: Vec<T>,
//         fixed_d: Vec<T>,
//         periodic_n: Vec<T>,
//         periodic_d: Vec<T>,
//     ) -> Self {
//         Self {
//             a0: T::one(),
//             b0: T::one(),
//             a1: fixed_d[0].clone(),
//             b1: T::one(),
//             numerators: Box::new(
//                 fixed_n
//                     .into_iter()
//                     .chain(periodic_n.into_iter().cycle())
//                     .skip(1),
//             ),
//             denominators: Box::new(
//                 fixed_d
//                     .into_iter()
//                     .chain(periodic_d.into_iter().cycle())
//                     .skip(1),
//             ),
//         }
//     }
// }

// impl<T: Clone + Integer + CheckedAdd + CheckedMul> Iterator for ContinuedFraction<T> {
//     type Item = Ratio<T>;

//     fn next(&mut self) -> Option<Self::Item> {
//         let out = Ratio::new(self.a1.clone(), self.b1.clone());

//         let n = self.numerators.next()?;
//         let d = self.denominators.next()?;

//         let a2 = d
//             .checked_mul(&self.a1)?
//             .checked_add(&n.checked_mul(&self.a0)?)?;
//         let b2 = d
//             .checked_mul(&self.b1)?
//             .checked_add(&n.checked_mul(&self.b0)?)?;

//         self.a0 = self.a1.clone();
//         self.b0 = self.b1.clone();
//         self.a1 = a2;
//         self.b1 = b2;

//         Some(out)
//     }
// }

/// Produce the convergents of a simple continued fraction given an integer sequence representing the partial denominators. The numerators are always 1.
pub struct SimpleContinuedFraction<T> {
    a0: T,
    b0: T,
    a1: T,
    b1: T,
    dens: Box<dyn Iterator<Item = T>>,
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
        }
    }

    /// A simple continued fraction with some fixed starting denominators and then a periodic part.
    pub fn new_periodic(fixed: &[T], periodic: &[T]) -> Self {
        assert!(fixed.len() > 0);
        assert!(periodic.len() > 0);
        Self {
            a0: T::one(),
            b0: T::zero(),
            a1: fixed[0].clone(),
            b1: T::one(),
            dens: Box::new(
                fixed
                    .to_vec()
                    .into_iter()
                    .skip(1)
                    .chain(periodic.to_vec().into_iter().cycle()),
            ),
        }
    }

    /// A simple continued fraction with a finite number of terms.
    pub fn new_finite(dens: &[T]) -> Self {
        assert!(dens.len() > 0);
        let mut p = dens.to_vec();
        p.push(T::zero());
        Self {
            a0: T::one(),
            b0: T::zero(),
            a1: p[0].clone(),
            b1: T::one(),
            dens: Box::new(p.into_iter().skip(1)),
        }
    }
}

impl<T: Clone + Integer + CheckedAdd + CheckedMul> Iterator for SimpleContinuedFraction<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = Ratio::new(self.a1.clone(), self.b1.clone());

        let d = self.dens.next()?;

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
use crate::core::rational_digits::rational_decimal_string as rds;
print_sequences!(
    SimpleContinuedFraction::new_periodic(&[1], &[1]).map(|q| rds(q, 5).unwrap()), 10; // Converges on phi
    SimpleContinuedFraction::new_periodic(&[1], &[2]).map(|q| rds(q, 5).unwrap()), 10; // Cnverges on sqrt(2)
    SimpleContinuedFraction::new_finite(&[3, 7, 15, 1, 292, 1]).map(|q| rds(q, 7).unwrap()), 10; // Cnverges on pi, notice the jump in accuracy when the 292 term is reached
    SimpleContinuedFraction::new_finite(&[3, 7, 15, 1, 292, 1]), 10;
);
