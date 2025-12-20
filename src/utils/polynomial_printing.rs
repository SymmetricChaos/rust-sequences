use num::{One, Signed, Zero};
use std::fmt::Debug;
use std::fmt::Display;

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
        format!("{}", c.abs())
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
        format!("{:?}", c.abs())
    } else if n == 1 {
        format!("{:?}x", c.abs())
    } else {
        format!("{:?}x^{n}", c.abs())
    }
}
