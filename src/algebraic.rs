use num::{BigInt, One};

use crate::core::polynomial::polynomial_string_signed;

/// All integer valued polynomials as ordered by Cantor.
pub struct Algebraic {
    coef: Vec<BigInt>,
}

impl Algebraic {
    pub fn new() -> Self {
        Self {
            coef: vec![BigInt::one()],
        }
    }
}

impl Iterator for Algebraic {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let out = Some(polynomial_string_signed(&self.coef, false));
        // Something with integer partitions perhaps?
        out
    }
}
