use crate::utils::factorial::factorial;
use num::{BigInt, CheckedMul, Integer};

/// The number of de Brujin sequences for an alphabet with k symbols for each substring length n, starting at zero.
pub struct DeBrujin<T> {
    k: T,
    f: T,
    dividend: T,
    divisior: T,
}

impl<T: CheckedMul + Clone + Integer> DeBrujin<T> {
    /// All fixed width integers will overflow quickly due to a doubly exponential term.
    pub fn new(k: T) -> Self {
        let f = factorial(k.clone());
        Self {
            k,
            f,
            dividend: T::one(),
            divisior: T::one(),
        }
    }
}

impl DeBrujin<BigInt> {
    pub fn new_big<G>(k: G) -> Self
    where
        BigInt: From<G>,
    {
        Self::new(BigInt::from(k))
    }
}

impl<T: CheckedMul + Clone + Integer> Iterator for DeBrujin<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.dividend.clone() / self.divisior.clone();

        // Advance dividend
        let mut ex = self.divisior.clone();
        while !ex.is_zero() {
            self.dividend = self.dividend.checked_mul(&self.f)?;
            ex = ex - T::one();
        }

        // Advance the divisor
        self.divisior = self.divisior.checked_mul(&self.k)?;
        Some(out)
    }
}

crate::check_sequences!(
    DeBrujin::new_big(2), ["1", "1", "2", "16", "2048", "67108864", "144115188075855872", "1329227995784915872903807060280344576"];
);
