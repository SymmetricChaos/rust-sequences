use itertools::Itertools;
use num::BigInt;

/// Any recurrence of the form
/// a_x = c_0 * a_{x-n} + c_1 * a_{x-n-1} + c_2 * a{x-n-2}...
pub struct AdditiveLinear {
    vals: Vec<BigInt>,
    coefs: Vec<BigInt>,
}

impl AdditiveLinear {
    /// The simplest linear recurrence with two terms and two coefficients.
    pub fn new(a: i64, b: i64, p: i64, q: i64) -> Self {
        Self {
            vals: vec![BigInt::from(a), BigInt::from(b)],
            coefs: vec![BigInt::from(p), BigInt::from(q)],
        }
    }

    /// The simplest linear recurrence with two terms and two coefficients.
    pub fn new_big(a: BigInt, b: BigInt, p: BigInt, q: BigInt) -> Self {
        Self {
            vals: vec![a, b],
            coefs: vec![p, q],
        }
    }

    /// Linear recurrence with any number of terms and coefficients.
    /// Panics if inits.len() != coefs.len()
    pub fn new_from_slices(inits: &[i64], coefs: &[i64]) -> Self {
        assert_eq!(inits.len(), coefs.len());
        Self {
            vals: inits.iter().map(|x| BigInt::from(*x)).collect_vec(),
            coefs: coefs.iter().map(|x| BigInt::from(*x)).collect_vec(),
        }
    }

    /// Linear recurrence with any number of terms and coefficients.
    /// Panics if inits.len() != coefs.len()
    pub fn new_from_slices_big(inits: &[BigInt], coefs: &[BigInt]) -> Self {
        assert_eq!(inits.len(), coefs.len());
        Self {
            vals: inits.to_owned(),
            coefs: coefs.to_owned(),
        }
    }
}

impl Iterator for AdditiveLinear {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.vals[0].clone();
        let mut t = BigInt::from(0);
        for (v, c) in self.vals.iter().zip(self.coefs.iter()) {
            t += v * c;
        }
        self.vals[0] = t;
        self.vals.rotate_left(1);
        Some(out)
    }
}

/// Any recurrence of the form
/// a_x = f_0(a_{x-1}) + f_1(a_{x-2}) + f_3(a_{x-3})...
pub struct Additive {
    vals: Vec<BigInt>,
    funcs: Vec<Box<dyn Fn(&BigInt) -> BigInt>>,
}

impl Additive {
    pub fn new(
        a: i64,
        b: i64,
        p: Box<dyn Fn(&BigInt) -> BigInt>,
        q: Box<dyn Fn(&BigInt) -> BigInt>,
    ) -> Self {
        Self {
            vals: vec![BigInt::from(a), BigInt::from(b)],
            funcs: vec![p, q],
        }
    }

    pub fn new_big(
        a: BigInt,
        b: BigInt,
        p: Box<dyn Fn(&BigInt) -> BigInt>,
        q: Box<dyn Fn(&BigInt) -> BigInt>,
    ) -> Self {
        Self {
            vals: vec![a, b],
            funcs: vec![p, q],
        }
    }

    pub fn new_from_slices(inits: &[i64], funcs: Vec<Box<dyn Fn(&BigInt) -> BigInt>>) -> Self {
        Self {
            vals: inits.iter().map(|x| BigInt::from(*x)).collect_vec(),
            funcs,
        }
    }

    pub fn new_from_slices_big(
        inits: &[BigInt],
        funcs: Vec<Box<dyn Fn(&BigInt) -> BigInt>>,
    ) -> Self {
        Self {
            vals: inits.to_owned(),
            funcs,
        }
    }
}

impl Iterator for Additive {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.vals[0].clone();
        let mut t = BigInt::from(0);
        for (v, c) in self.vals.iter().zip(self.funcs.iter()) {
            t += c(v);
        }
        self.vals[0] = t;
        self.vals.rotate_left(1);
        Some(out)
    }
}

crate::print_a_few!(
    AdditiveLinear::new(0, 1, 2, 3), 0, 10;
    AdditiveLinear::new_from_slices(&[3, 0, 2], &[1, 1, 0]), 0, 10;
    Additive::new(0, 1, Box::new(|x| x + 1), Box::new(|x| x * -2)), 0, 10;
    Additive::new_from_slices(&[0, 1, 2], vec![Box::new(|x| x + 1), Box::new(|x| x * -2), Box::new(|x| x + x/2 + 4)]), 0, 10;
);
