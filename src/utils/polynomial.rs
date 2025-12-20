use num::{One, PrimInt, Signed, Zero};
use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Index, IndexMut, MulAssign, Neg},
};

use crate::utils::polynomial_printing::{polynomial_debug, polynomial_display};

pub const DEFAULT_TO_ASCENDING_DISPLAY: bool = true;

#[derive(Clone, PartialEq, Eq)]
pub struct Polynomial<N> {
    pub coef: Vec<N>,
}

// Implementation for any type
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

// Implementations for primitive integer types
impl<N: PrimInt + MulAssign + AddAssign> Polynomial<N> {
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

    /// A new polynomial with coefficient given in increasing order so that constant term is at index zero. All trailing zero coefficients are removed.
    pub fn new(coef: &[N]) -> Self {
        let mut p = Self {
            coef: coef.to_vec(),
        };
        p.trim();
        p
    }

    /// A new polynomial with coefficient given in increasing order so that constant term is at index zero.
    /// No trimming is performed. This potentially invalidates evaluation of any method that relies on knowing the degree of the polynomial.
    pub fn new_raw(coef: &[N]) -> Self {
        Self {
            coef: coef.to_vec(),
        }
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

    /// Evaluate the polynomial at a point, panicking on overflow.
    pub fn eval(&self, n: &N) -> N {
        let mut out = <N>::zero();
        let mut x = <N>::one();
        for i in self.iter() {
            out += *i * *n;
            x *= *n;
        }
        out
    }

    /// Evaluate the polynomial at a point, returning None on overflow.
    pub fn eval_chekced(&self, n: &N) -> Option<N> {
        let mut out = <N>::zero();
        let mut x = <N>::one();
        for i in self.iter() {
            out = out.checked_add(&i.checked_mul(n)?)?;
            x = x.checked_mul(n)?;
        }
        Some(out)
    }
}

impl<N: PrimInt + AddAssign + MulAssign> Add for Polynomial<N> {
    type Output = Polynomial<N>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut p = if self.len() >= rhs.len() {
            let mut out = self.coef.clone();
            for (i, c) in rhs.coef.iter().enumerate() {
                out[i] = out[i] + *c;
            }
            Polynomial::new(&out)
        } else {
            let mut out = rhs.coef.clone();
            for (i, c) in self.coef.iter().enumerate() {
                out[i] = out[i] + *c;
            }
            Polynomial::new(&out)
        };
        p.trim();
        p
    }
}

impl<N: PrimInt + AddAssign + MulAssign> AddAssign for Polynomial<N> {
    fn add_assign(&mut self, rhs: Self) {
        if self.len() >= rhs.len() {
            for (i, c) in rhs.coef.iter().enumerate() {
                self.coef[i] += *c;
            }
        } else {
            while self.len() < rhs.len() {
                self.coef.push(N::zero());
            }
            for (i, c) in rhs.coef.iter().enumerate() {
                self.coef[i] += *c;
            }
        }
    }
}

impl<N: PrimInt + Signed> Neg for Polynomial<N> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        for i in self.coef.iter_mut() {
            *i = -*i;
        }
        self
    }
}

macro_rules! poly_extras_unsigned {
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
                        .fold(<$t>::zero(), |acc, x| acc + x)
                        .try_into()
                        .ok()?;
                    Some(self.degree()? + s - 1)
                }
            }
        }
    };
}

macro_rules! poly_extras_signed {
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

poly_extras_signed!(i64);
poly_extras_unsigned!(u64);
poly_extras_signed!(i32);
poly_extras_unsigned!(u32);

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

#[cfg(test)]
mod polynomial_tests {
    use super::*;
    #[test]
    fn polynomial_struct() {
        let mut p = Polynomial::new_raw(&[0_i64, 1234, 0, -166, -1, 94, 0]);
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

        let q = Polynomial::new(&[0_i64, 0, 0]);
        assert!(q.is_constant());

        let r = Polynomial::new(&[1_i64, 1]);
        assert_eq!(r.cantor_height().unwrap(), 2);
    }

    #[test]
    fn polynomial_ordering_test() {
        let p = Polynomial::new(&[0, 1234, 0, -166, -1, 94, 0]);

        assert_eq!(
            format!("{}", p.to_string_descending()),
            "94x^5 - x^4 - 166x^3 + 1234x"
        );
        assert_eq!(
            format!("{}", p.to_string_ascending()),
            "1234x - 166x^3 - x^4 + 94x^5"
        );
    }
}
