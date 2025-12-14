use std::fmt::{Debug, Display};

use num::{Integer, One, Signed, Zero};

pub struct Polynomial<N> {
    coef: Vec<N>,
}

impl<N: Clone + Integer + Signed> Polynomial<N>
where
    usize: TryFrom<N>,
{
    /// A new polynomial with coefficient given in increasing order so that constant term is at index zero. All trailing zero coefficients are removed.
    pub fn new(coef: &[N]) -> Self {
        let mut p = Self {
            coef: coef.to_vec(),
        };
        p.trim();
        p
    }

    /// A new polynomial with coefficient given in increasing order so that constant term is at index zero.
    /// No trimming is performed. This invalidates evaluation of .height(), .is_constant(), etc
    pub fn new_raw(coef: &[N]) -> Self {
        Self {
            coef: coef.to_vec(),
        }
    }

    /// Remove all trailing zero coefficients.
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

    /// Determine if the polynomial is a constant. Assumes the polynomial is trimmed.
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

    /// Cantor's height function. The degree of the polynomial plus the sum of the absolute values of the coefficients minus one. None if the polynomial is all zero.
    pub fn cantor_height(&self) -> Option<usize> {
        if self.coef.is_empty() {
            None
        } else {
            let s: usize = self
                .coef
                .iter()
                .fold(N::zero(), |acc, x| acc + x.abs())
                .try_into()
                .ok()?;
            Some(self.degree()? + s - 1)
        }
    }
}

impl<N: Clone + Display + Zero + One + PartialEq + Signed> Display for Polynomial<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", polynomial_display(&self.coef))
    }
}

impl<N: Clone + Debug + Zero + One + PartialEq + Signed> Debug for Polynomial<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", polynomial_debug(&self.coef))
    }
}

/// A polynomial displayed in ascending order
pub fn polynomial_display<N: Display + Zero + One + PartialEq + Signed>(
    polynomial: &[N],
) -> String {
    if polynomial.is_empty() {
        return N::zero().to_string();
    }

    let mut out = String::new();
    let mut coefs = polynomial
        .iter()
        .enumerate()
        .skip_while(|(_, c)| c.is_zero());

    match coefs.next() {
        Some((n, c)) => out.push_str(&first_term_str_display(c, n)),
        None => return N::zero().to_string(),
    };
    for (n, c) in coefs {
        if c.is_zero() {
            continue;
        }
        if c.is_negative() {
            out.push_str(" - ");
        } else {
            out.push_str(" + ");
        }
        out.push_str(&term_str_display(c, n))
    }

    out
}

fn first_term_str_display<N: Display + Zero + One + PartialEq + Signed>(c: &N, n: usize) -> String {
    if n == 0 {
        format!("{}", c)
    } else if n == 1 {
        if c.abs().is_one() {
            format!("x")
        } else {
            format!("{c}x")
        }
    } else {
        if c.abs().is_one() {
            format!("x^{n}")
        } else {
            format!("{c}x^{n}")
        }
    }
}

fn term_str_display<N: Display + Zero + One + PartialEq + Signed>(c: &N, n: usize) -> String {
    if n == 0 {
        format!("{}", c)
    } else if n == 1 {
        if c.abs().is_one() {
            format!("x")
        } else {
            format!("{}x", c.abs())
        }
    } else {
        if c.abs().is_one() {
            format!("x^{n}")
        } else {
            format!("{}x^{n}", c.abs())
        }
    }
}

pub fn polynomial_debug<N: Debug + Zero + One + PartialEq + Signed>(polynomial: &[N]) -> String {
    if polynomial.is_empty() {
        return format!("{:?}", N::zero());
    }

    let mut out = String::new();
    let mut coefs = polynomial.iter().enumerate();

    match coefs.next() {
        Some((n, c)) => out.push_str(&first_term_str_debug(c, n)),
        None => return format!("{:?}", N::zero()),
    };
    for (n, c) in coefs {
        // In debug we will show any zeroes as positive terms
        if c.is_negative() {
            out.push_str(" - ");
        } else {
            out.push_str(" + ");
        }
        out.push_str(&term_str_debug(c, n))
    }

    out
}

fn first_term_str_debug<N: Debug + Zero + One + PartialEq + Signed>(c: &N, n: usize) -> String {
    if n == 0 {
        format!("{c:?}")
    } else if n == 1 {
        if c.abs().is_one() {
            format!("x")
        } else {
            format!("{c:?}x")
        }
    } else {
        if c.abs().is_one() {
            format!("x^{n}")
        } else {
            format!("{c:?}x^{n}")
        }
    }
}

fn term_str_debug<N: Debug + Zero + One + PartialEq + Signed>(c: &N, n: usize) -> String {
    if n == 0 {
        format!("{c:?}")
    } else if n == 1 {
        if c.abs().is_one() {
            format!("x")
        } else {
            format!("{:?}x", c.abs())
        }
    } else {
        if c.abs().is_one() {
            format!("x^{n}")
        } else {
            format!("{:?}x^{n}", c.abs())
        }
    }
}

#[cfg(test)]
mod polynomial_tests {
    use super::*;
    #[test]
    fn polynomial_struct() {
        let mut p = Polynomial::new_raw(&[0, 1234, 0, -166, -1, 94, 0]);
        assert_eq!(p.to_string(), "1234x - 166x^3 - x^4 + 94x^5");
        assert_eq!(
            format!("{:?}", p),
            "0 + 1234x + 0x^2 - 166x^3 - x^4 + 94x^5 + 0x^6"
        );
        p.trim();
        assert_eq!(
            format!("{:?}", p),
            "0 + 1234x + 0x^2 - 166x^3 - x^4 + 94x^5"
        );
        assert!(!p.is_constant());

        let q = Polynomial::new(&[0, 0, 0]);
        assert!(q.is_constant());

        let r = Polynomial::new(&[1, 1]);
        assert_eq!(r.cantor_height().unwrap(), 2);
    }
}
