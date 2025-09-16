use num::{BigInt, CheckedAdd, CheckedMul, One, PrimInt};

/// The factorial numbers.
/// 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800...
pub struct Factorial<T> {
    val: T,
    ctr: T,
}

impl<T: PrimInt> Factorial<T> {
    pub fn new_prim() -> Self {
        Self {
            val: T::one(),
            ctr: T::one() + T::one(),
        }
    }
}

impl Factorial<BigInt> {
    pub fn new() -> Self {
        Self {
            val: BigInt::from(1),
            ctr: BigInt::from(2),
        }
    }
}

impl<T: CheckedAdd + CheckedMul + Clone + One> Iterator for Factorial<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.val.checked_mul(&self.ctr)?;
        self.ctr = self.ctr.checked_mul(&T::one())?;
        Some(out)
    }
}

crate::check_iteration_times!(
    Factorial::new(), 37_000;
);

crate::check_sequences!(
    Factorial::new(), 0, 10, [1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800];
);
