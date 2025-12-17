use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign},
};

use num::{One, Signed, Zero};

pub const DEFAULT_TO_ASCENDING_DISPLAY: bool = true;

pub struct Polynomial<N> {
    pub coef: Vec<N>,
}

impl<N> Polynomial<N> {
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
}

impl<N: Clone + Zero> Polynomial<N> {
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
}

impl<N: Clone + Signed + Zero> Polynomial<N>
where
    usize: TryFrom<N>,
{
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

impl<N: Clone + Display + One + PartialEq + Signed + Zero> Polynomial<N> {
    pub fn to_string_ascending(&self) -> String {
        polynomial_display(&self.coef, true)
    }

    pub fn to_string_descending(&self) -> String {
        polynomial_display(&self.coef, false)
    }
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

impl<N: Add + Clone + Zero> Add for Polynomial<N> {
    type Output = Polynomial<N>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.len() >= rhs.len() {
            let mut out = self.coef.clone();
            for (i, c) in rhs.coef.iter().enumerate() {
                out[i] = out[i].clone() + c.clone();
            }
            Polynomial::new(&out)
        } else {
            let mut out = rhs.coef.clone();
            for (i, c) in self.coef.iter().enumerate() {
                out[i] = out[i].clone() + c.clone();
            }
            Polynomial::new(&out)
        }
    }
}

pub fn polynomial_display<N: Display + Zero + One + PartialEq + Signed>(
    polynomial: &[N],
    ascending: bool,
) -> String {
    if polynomial.is_empty() {
        return N::zero().to_string();
    }

    let mut out = String::new();
    if ascending {
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
    } else {
        let mut coefs = polynomial
            .iter()
            .enumerate()
            .rev()
            .skip_while(|(_, c)| c.is_zero());

        match coefs.next() {
            Some((n, c)) => out.push_str(&first_term_str_display(c, n)),
            None => return format!("{}", N::zero()),
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
    };

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

pub fn polynomial_debug<N: Debug + Zero + One + PartialEq + Signed>(
    polynomial: &[N],
    ascending: bool,
) -> String {
    if polynomial.is_empty() {
        return format!("{:?}", N::zero());
    }

    let mut out = String::new();

    let mut coefs = polynomial.iter().enumerate();

    if ascending {
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
    }

    out
}

fn first_term_str_debug<N: Debug + Zero + One + PartialEq>(c: &N, n: usize) -> String {
    if n == 0 {
        format!("{c:?}")
    } else if n == 1 {
        format!("{c:?}x")
    } else {
        format!("{c:?}x^{n}")
    }
}

fn term_str_debug<N: Debug + Zero + One + PartialEq + Signed>(c: &N, n: usize) -> String {
    if n == 0 {
        format!("{c:?}")
    } else if n == 1 {
        format!("{:?}x", c.abs())
    } else {
        format!("{:?}x^{n}", c.abs())
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
            "0 + 1234x + 0x^2 - 166x^3 - 1x^4 + 94x^5 + 0x^6"
        );
        p.trim();
        assert_eq!(
            format!("{:?}", p),
            "0 + 1234x + 0x^2 - 166x^3 - 1x^4 + 94x^5"
        );
        assert!(!p.is_constant());

        let q = Polynomial::new(&[0, 0, 0]);
        assert!(q.is_constant());

        let r = Polynomial::new(&[1, 1]);
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
