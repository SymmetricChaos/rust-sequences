use num::{BigInt, CheckedAdd, Integer};

/// A Weyl-Marsaglia sequence starting at zero. A permutation of all integers 0..n that repeats every n values.
pub struct WeylMarsaglia<T> {
    n: T,
    step: T,
    modulus: T,
}

impl<T: CheckedAdd + Clone + Integer> WeylMarsaglia<T> {
    /// Create a new sequence with the given step and modulus.
    /// Panics if step is not coprime to modulus.
    pub fn new(step: T, modulus: T) -> Self {
        assert!(modulus != T::zero(), "modulus cannot be zero");
        assert!(
            step.gcd(&modulus) == T::one(),
            "step must be copriem to modulus"
        );
        Self {
            n: T::zero(),
            step,
            modulus,
        }
    }
}

impl WeylMarsaglia<BigInt> {
    /// Create a new sequence with the given step and modulus.
    /// Panics if step is not coprime to modulus.
    pub fn new_big<N>(step: N, modulus: N) -> Self
    where
        BigInt: From<N>,
    {
        Self::new(BigInt::from(step), BigInt::from(modulus))
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
