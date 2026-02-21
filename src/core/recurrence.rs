use itertools::Itertools;
use num::{BigInt, CheckedAdd, CheckedMul, Zero};

/// Any recurrence of the form
/// a_x = c_0 * a_{x-n} + c_1 * a_{x-n-1} + c_2 * a{x-n-2}...
pub struct AdditiveLinearRecurrence<T> {
    vals: Vec<T>,
    coefs: Vec<T>,
}

impl<T: CheckedAdd + CheckedMul + Clone + Zero> AdditiveLinearRecurrence<T> {
    /// The simplest linear recurrence with two terms and two coefficients.
    pub fn new(a: T, b: T, p: T, q: T) -> Self {
        Self {
            vals: vec![a, b],
            coefs: vec![p, q],
        }
    }

    /// Linear recurrence with any number of terms and coefficients.
    /// Panics if inits.len() != coefs.len()
    pub fn new_from_slices(inits: &[T], coefs: &[T]) -> Self {
        assert_eq!(inits.len(), coefs.len());
        Self {
            vals: inits.iter().cloned().collect_vec(),
            coefs: coefs.iter().cloned().collect_vec(),
        }
    }
}

impl AdditiveLinearRecurrence<BigInt> {
    /// The simplest linear recurrence with two terms and two coefficients.
    pub fn new_big<T>(a: T, b: T, p: T, q: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            vals: vec![BigInt::from(a), BigInt::from(b)],
            coefs: vec![BigInt::from(p), BigInt::from(q)],
        }
    }

    /// Linear recurrence with any number of terms and coefficients.
    /// Panics if inits.len() != coefs.len()
    pub fn new_big_from_slices<T: Clone>(inits: &[T], coefs: &[T]) -> Self
    where
        BigInt: From<T>,
    {
        assert_eq!(inits.len(), coefs.len());
        Self {
            vals: inits
                .into_iter()
                .map(|x| BigInt::from(x.clone()))
                .collect_vec(),
            coefs: coefs
                .into_iter()
                .map(|x| BigInt::from(x.clone()))
                .collect_vec(),
        }
    }
}

impl<T: CheckedAdd + CheckedMul + Clone + Zero> Iterator for AdditiveLinearRecurrence<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.vals[0].clone();
        let mut t = T::zero();
        for (v, c) in self.vals.iter().zip(self.coefs.iter()) {
            t = t.checked_add(&v.checked_mul(c)?)?;
        }
        self.vals[0] = t;
        self.vals.rotate_left(1);
        Some(out)
    }
}

/// Any recurrence of the form
/// a_x = f_0(a_{x-1}) + f_1(a_{x-2}) + f_3(a_{x-3})...
pub struct AdditiveRecurrence {
    vals: Vec<BigInt>,
    funcs: Vec<Box<dyn Fn(&BigInt) -> BigInt>>,
}

impl AdditiveRecurrence {
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

    pub fn new_big_from_slices(inits: &[i64], funcs: Vec<Box<dyn Fn(&BigInt) -> BigInt>>) -> Self {
        Self {
            vals: inits.iter().map(|x| BigInt::from(*x)).collect_vec(),
            funcs,
        }
    }
}

impl Iterator for AdditiveRecurrence {
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

crate::print_sequences!(
    AdditiveLinearRecurrence::new(0, 1, 2, 3), 0, 10;
    AdditiveLinearRecurrence::new_big_from_slices(&[3, 0, 2], &[1, 1, 0]), 0, 10;
    // AdditiveRecurrence::new_big(0, 1, Box::new(|x| x + 1), Box::new(|x| x * -2)), 0, 10;
    // AdditiveRecurrence::new_big_from_slices(&[0, 1, 2], vec![Box::new(|x| x + 1), Box::new(|x| x * -2), Box::new(|x| x + x/2 + 4)]), 0, 10;
);
