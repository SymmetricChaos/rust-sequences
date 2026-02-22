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

// impl<T: Clone + Integer + CheckedAdd + CheckedMul + Display> Iterator for ContinuedFraction<T> {
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

    pub fn new_periodic(fixed: T, periodic: &[T]) -> Self {
        Self {
            a0: T::one(),
            b0: T::zero(),
            a1: fixed,
            b1: T::one(),
            dens: Box::new(periodic.to_vec().into_iter().cycle()),
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
        self.a1 = a2;
        self.b1 = b2;

        Some(out)
    }
}

#[cfg(test)]
use crate::core::rational_digits::rational_decimal_string;
print_sequences!(
    SimpleContinuedFraction::new_periodic(1, &[1]).map(|q| rational_decimal_string(q, 5).unwrap()), 0, 10; // Converges on phi
    SimpleContinuedFraction::new_periodic(1, &[2]).map(|q| rational_decimal_string(q, 5).unwrap()), 0, 10; // Cnverges on sqrt(2)
);
