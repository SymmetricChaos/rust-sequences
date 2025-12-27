use num::{One, Signed, Zero};
use std::{
    fmt::{Debug, Display},
    ops::{Index, IndexMut},
};

use crate::utils::polynomial_printing::{polynomial_debug, polynomial_display};

pub const DEFAULT_TO_ASCENDING_DISPLAY: bool = true;

/// A univariate polynomials
#[derive(Clone, PartialEq, Eq)]
pub struct Polynomial<N> {
    pub coef: Vec<N>,
}

impl<N> Index<usize> for Polynomial<N> {
    type Output = N;

    fn index(&self, index: usize) -> &Self::Output {
        &self.coef[index]
    }
}
impl<N> IndexMut<usize> for Polynomial<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.coef[index]
    }
}

impl<N> Polynomial<N> {
    /// A new polynomial with coefficients given in increasing order so that constant term is at index zero.
    /// No trimming is performed. This potentially invalidates evaluation of any method that relies on knowing the degree of the polynomial.
    pub fn new_raw(coef: Vec<N>) -> Self {
        Self { coef }
    }

    /// Determine if the polynomial is a constant.
    pub fn is_constant(&self) -> bool {
        self.coef.len() <= 1
    }

    /// The degree of a polynomial. None if the polynomial is all zero.
    pub fn degree(&self) -> Option<usize> {
        if self.coef.is_empty() {
            None
        } else {
            Some(self.coef.len() - 1)
        }
    }

    /// Returns a reference to the coefficient at the index or None if out of bounds.
    pub fn get(&self, index: usize) -> Option<&N> {
        self.coef.get(index)
    }

    /// Returns a mutable reference to the coefficient at the index or None if out of bounds.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut N> {
        self.coef.get_mut(index)
    }

    /// Length of the underlying vector
    pub fn len(&self) -> usize {
        self.coef.len()
    }

    /// Iterate over the coefficients in ascending order with immutable references.
    pub fn iter(&self) -> std::slice::Iter<'_, N> {
        self.coef.iter()
    }

    /// Iterate over the coefficients in ascending order with mutable references.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, N> {
        self.coef.iter_mut()
    }
}

impl<N: Clone + Zero> Polynomial<N> {
    /// A new polynomial with coefficients given in increasing order so that constant term is at index zero. All trailing zero coefficients are removed.
    pub fn new(coef: Vec<N>) -> Self {
        let mut p = Self { coef };
        p.trim();
        p
    }

    /// Remove all trailing zero coefficients. This is performed automatically after operations that may change it.
    pub fn trim(&mut self) {
        let mut last = self.coef.len() - 1;
        loop {
            if self.coef[last].is_zero() && last > 0 {
                last -= 1;
            } else {
                break;
            }
        }
        self.coef.truncate(last + 1);
    }

    /// Get irrefutable. Returns a clone of the coefficient or zero if the value is too high.
    pub fn get_irref(&self, n: usize) -> N {
        match self.get(n) {
            Some(n) => n.clone(),
            None => N::zero(),
        }
    }

    /// Increase the degree of the polynomial as if multipying by x^n.
    pub fn increase_degree(&mut self, n: usize) {
        for _ in 0..n {
            self.coef.insert(0, N::zero());
        }
    }
}

// Printing methods
impl<N: Clone + Display + One + PartialEq + Signed + Zero> Polynomial<N> {
    /// Print the polynomial with coefficients in ascending order.
    pub fn to_string_ascending(&self) -> String {
        polynomial_display(&self.coef, true)
    }

    /// Print the polynomial with coefficients in descending order.
    pub fn to_string_descending(&self) -> String {
        polynomial_display(&self.coef, false)
    }
}
impl<N: Clone + Debug + One + PartialEq + Signed + Zero> Polynomial<N> {
    /// Print the polynomial with coefficients in ascending order, showing all coefficients in debug format.
    pub fn debug_string_ascending(&self) -> String {
        polynomial_debug(&self.coef, true)
    }

    /// Print the polynomial with coefficients in descending order, showing all coefficients in debug format.
    pub fn debug_string_descending(&self) -> String {
        polynomial_debug(&self.coef, false)
    }
}
impl<N: Clone + Display + One + PartialEq + Signed + Zero> Display for Polynomial<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            polynomial_display(&self.coef, DEFAULT_TO_ASCENDING_DISPLAY)
        )
    }
}
impl<N: Clone + Debug + One + PartialEq + Signed + Zero> Debug for Polynomial<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            polynomial_debug(&self.coef, DEFAULT_TO_ASCENDING_DISPLAY)
        )
    }
}

macro_rules! poly_extras {
    ($t:ty) => {
        impl Polynomial<$t> {
            /// Cantor's height function. The degree of the polynomial plus the sum of the absolute values of the coefficients minus one. None if the polynomial is all zero.
            pub fn cantor_height(&self) -> Option<usize> {
                if self.coef.is_empty() {
                    None
                } else {
                    let s: usize = self
                        .coef
                        .iter()
                        .fold(<$t>::zero(), |acc, x| acc + x.abs())
                        .try_into()
                        .ok()?;
                    Some(self.degree()? + s - 1)
                }
            }
        }
    };
}

poly_extras!(i64);
poly_extras!(i32);

#[cfg(test)]
mod polynomial_tests {
    use num::rational::Ratio;

    use super::*;
    #[test]
    fn polynomial_struct() {
        let mut p = Polynomial::new_raw(vec![0_i64, 1234, 0, -166, -1, 94, 0]);
        assert_eq!(p.to_string(), "1234x - 166x^3 - x^4 + 94x^5");
        assert_eq!(
            format!("{:?}", p),
            "0 + 1234x + 0x^2 - 166x^3 - 1x^4 + 94x^5 + 0x^6"
        );
        p.trim();
        assert_eq!(
            format!("{:?}", p),
            "0 + 1234x + 0x^2 - 166x^3 - 1x^4 + 94x^5"
        );
        assert!(!p.is_constant());

        let q: Polynomial<i64> = Polynomial::new(vec![0_i64, 0, 0]);
        assert!(q.is_constant());

        let r: Polynomial<i64> = Polynomial::new(vec![1_i64, 1]);
        assert_eq!(r.cantor_height().unwrap(), 2);

        let r: Polynomial<Ratio<i32>> =
            Polynomial::new(vec![Ratio::new(1, 2), Ratio::new(-3, 7), Ratio::new(2, 1)]);
        assert_eq!(format!("{}", r), "1/2 - 3/7x + 2x^2");

        assert_eq!(4132, p.eval(&2));
        assert_eq!(Ratio::new(109, 126), r.eval(&Ratio::new(-1, 3)));
    }

    #[test]
    fn polynomial_ordering_test() {
        let p = Polynomial::new(vec![0, 1234, 0, -166, -1, -94, 0]);

        assert_eq!(
            format!("{}", p.to_string_descending()),
            "-94x^5 - x^4 - 166x^3 + 1234x"
        );
        assert_eq!(
            format!("{}", p.to_string_ascending()),
            "1234x - 166x^3 - x^4 + -94x^5"
        );
    }

    #[test]
    fn poylnomial_addition() {
        let p = Polynomial::new(vec![0, 1, 2, 3, 4, 5]);
        let q = Polynomial::new(vec![1, 3, 5, 7, 9, 11]);
        let z: Polynomial<i32> = &p + &q;
        println!(
            "({}) +\n({}) =\n{}",
            p.to_string_descending(),
            q.to_string_descending(),
            z.to_string_descending()
        );
        assert_eq!(q, &z - &p);
    }

    #[test]
    fn poylnomial_multiplication() {
        let p = Polynomial::new(vec![0, 1, 2, 3, 4, 5]);
        let q = Polynomial::new(vec![1, 3, 5, 7, 9, 11]);
        let z: Polynomial<i32> = &p * &q;
        println!(
            "({}) +\n({}) =\n{}",
            p.to_string_descending(),
            q.to_string_descending(),
            z.to_string_descending()
        );
    }
}
