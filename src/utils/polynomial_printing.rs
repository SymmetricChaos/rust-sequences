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
            if c.is_negative() {
                format!("-x")
            } else {
                format!("x")
            }
        } else {
            format!("{c}x")
        }
    } else {
        if c.abs().is_one() {
            if c.is_negative() {
                format!("-x^{n}")
            } else {
                format!("x^{n}")
            }
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

    if ascending {
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
    } else {
        let mut coefs = polynomial.iter().enumerate().rev();
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

#[cfg(test)]
mod polynomial_tests {

    use crate::utils::polynomial::Polynomial;

    #[test]
    fn polynomial_display_cheks() {
        let polys = [
            (vec![2, 3, 4], "2 + 3x + 4x^2", "4x^2 + 3x + 2"),
            (vec![-2, -3, -4], "-2 - 3x - 4x^2", "-4x^2 - 3x - 2"),
            (vec![0, 2, 3], "2x + 3x^2", "3x^2 + 2x"),
            (vec![0, -2, -3], "-2x - 3x^2", "-3x^2 - 2x"),
            (vec![1, 2, 3], "1 + 2x + 3x^2", "3x^2 + 2x + 1"),
            (vec![-1, 2, 3], "-1 + 2x + 3x^2", "3x^2 + 2x - 1"),
            (vec![2, 1, 3], "2 + x + 3x^2", "3x^2 + x + 2"),
            (vec![2, -1, 3], "2 - x + 3x^2", "3x^2 - x + 2"),
            (vec![2, -1, -3], "2 - x - 3x^2", "-3x^2 - x + 2"),
            (vec![2, 0, 3], "2 + 3x^2", "3x^2 + 2"),
            (vec![2, 0, -3], "2 - 3x^2", "-3x^2 + 2"),
            (vec![0, -1, 3], "-x + 3x^2", "3x^2 - x"),
            (vec![0, 0, -1, 2], "-x^2 + 2x^3", "2x^3 - x^2"),
            (vec![0, 0, -2, 3], "-2x^2 + 3x^3", "3x^3 - 2x^2"),
        ];

        for (coef, asc, des) in polys {
            let pol = Polynomial::new(coef);
            assert_eq!(asc, pol.to_string_ascending());
            assert_eq!(des, pol.to_string_descending());
        }

        // for (coef, _, _) in polys {
        //     let pol = Polynomial::new(coef);
        //     println!("{}", pol.debug_string_ascending());
        //     println!("{}", pol.debug_string_descending());
        // }
    }
}
