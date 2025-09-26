use num::{BigInt, CheckedAdd, Integer, PrimInt, Zero, traits::CheckedRem};

/// A Weyl-Marsaglia sequence starting at zero. A permutation of all integers 0..n that repeats every n values.
pub struct WeylMarsaglia<T> {
    n: T,
    step: T,
    modulus: T,
}

impl<T: PrimInt + Integer> WeylMarsaglia<T> {
    /// Create a new sequence with the given step and modulus.
    /// Panics if step is not coprime to modulus.
    pub fn new(step: T, modulus: T) -> Self {
        assert!(step.gcd(&modulus) == T::one());
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
        Self {
            n: BigInt::zero(),
            step: BigInt::from(step),
            modulus: BigInt::from(modulus),
        }
    }
}

impl<T: CheckedAdd + CheckedRem + Clone> Iterator for WeylMarsaglia<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.n.clone();
        self.n = self.n.checked_add(&self.step)?;
        self.n = self.n.checked_rem(&self.modulus)?;
        Some(out)
    }
}

crate::print_values!(
    WeylMarsaglia::new(5, 16), 0, 17;
);
