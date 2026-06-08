use crate::Number;
use num::{BigInt, CheckedAdd, Integer, One, Signed};

/// A Weyl-Marsaglia sequence starting at zero. A permutation of all integers 0..n that repeats every n values.
///
/// ```text
/// For step = 5 amd modulus = 16:
/// 0, 5, 10, 15, 4, 9, 14, 3, 8, 13, 2, 7, 12, 1, 6, 11, 0
/// ```
pub struct WeylMarsaglia<T> {
    n: T,
    step: T,
    modulus: T,
}

impl WeylMarsaglia<Number> {
    /// Create a new sequence with the given step and modulus.
    /// Panics if step is not coprime to modulus.
    pub fn new(step: Number, modulus: Number) -> Self {
        assert!(modulus.is_positive(), "modulus must be positive");
        assert!(step.gcd(&modulus) == 1, "step must be coprime to modulus");
        Self {
            n: 1,
            step,
            modulus,
        }
    }
}

#[cfg(feature = "big_int")]
impl WeylMarsaglia<BigInt> {
    /// Create a new sequence with the given step and modulus.
    /// Panics if step is not coprime to modulus.
    pub fn new_big<N>(step: N, modulus: N) -> Self
    where
        BigInt: From<N>,
    {
        let step = BigInt::from(step);
        let modulus = BigInt::from(modulus);
        assert!(modulus.is_positive(), "modulus must be positive");
        assert!(
            step.gcd(&modulus).is_one(),
            "step must be coprime to modulus"
        );
        Self {
            n: BigInt::one(),
            step,
            modulus,
        }
    }
}

impl<T: CheckedAdd + Clone + Integer> Iterator for WeylMarsaglia<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.n.clone();
        self.n = self.n.checked_add(&self.step)?;
        self.n = self.n.clone() % self.modulus.clone();
        Some(out)
    }
}

crate::print_sequences!(
    WeylMarsaglia::new(5, 16), 17;
    WeylMarsaglia::new_big(5, 16), 17;
);
