use std::fmt::Display;

use num::{One, Signed, Zero};

pub fn polynomial_string_signed<N: Display + Zero + One + PartialEq + Signed>(
    polynomial: &[N],
    ascending: bool,
) -> String {
    if polynomial.is_empty() {
        return N::zero().to_string();
    }

    let mut out = String::new();
    let mut coefs = polynomial
        .iter()
        .enumerate()
        .skip_while(|(_, c)| c.is_zero());

    if ascending {
        match coefs.next() {
            Some((n, c)) => out.push_str(&first_term_str_signed(c, n)),
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
            out.push_str(&term_str_signed(c, n))
        }
    } else {
        let m = polynomial.len() - 1;

        match coefs.next() {
            Some((n, c)) => out.push_str(&first_term_str_signed(c, m - n)),
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
            out.push_str(&term_str_signed(c, m - n))
        }
    }
    out
}

fn first_term_str_signed<N: Display + Zero + One + PartialEq + Signed>(c: &N, n: usize) -> String {
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

fn term_str_signed<N: Display + Zero + One + PartialEq + Signed>(c: &N, n: usize) -> String {
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

#[cfg(test)]
mod polynomial_tests {
    use super::*;

    #[test]
    fn polynomial_display() {
        assert_eq!(
            polynomial_string_signed(&[0, 1234, 0, -166, -1, 94, 0], true),
            "1234x - 166x^3 - x^4 + 94x^5"
        );
        assert_eq!(
            polynomial_string_signed(&[0, -1234, 0, -166, 1, 94, 0], false),
            "-1234x^5 - 166x^3 + x^2 + 94x"
        );
    }
}
