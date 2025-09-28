use num::{CheckedAdd, CheckedMul, Integer, One, rational::Ratio};

pub struct WallisProduct<T> {
    ctr: T,
    product: Ratio<T>,
}

impl<T: Clone + Integer + One> WallisProduct<T> {
    pub fn new() -> Self {
        Self {
            ctr: T::one(),
            product: Ratio::<T>::one(),
        }
    }
}

impl<T: Clone + Integer + CheckedMul + CheckedAdd> Iterator for WallisProduct<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.product.clone();
        let four = T::one() + T::one() + T::one() + T::one();
        let four_sq = four * self.ctr.clone() * self.ctr.clone();
        let term = Ratio::new(four_sq.clone(), four_sq.clone() - T::one());
        self.product = self.product.checked_mul(&term)?;
        self.ctr = self.ctr.checked_add(&T::one())?;
        Some(out.checked_add(&out)?)
    }
}

#[cfg(test)]
use crate::core::rational_digits::DecimalDigits;
#[cfg(test)]
use itertools::Itertools;
#[cfg(test)]
use num::BigInt;

crate::print_values!(
    WallisProduct::<u64>::new(), 0, 10;
    WallisProduct::<BigInt>::new().map(|x| DecimalDigits::from_ratio_big(x).map(|d| d.to_string()).take(6).join("")), 0, 20;
);
