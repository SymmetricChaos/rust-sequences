use crate::core::rational_decimal_string;
use num::{BigInt, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Integer, One, rational::Ratio};
use std::fmt::Display;

/// Convenience trait for counting one and zeroes.
pub trait CountBits {
    fn count_ones_64(&self) -> u64;
    fn count_zeros_64(&self) -> u64;
}

impl CountBits for BigInt {
    fn count_ones_64(&self) -> u64 {
        let mut out = 0;
        for i in self.iter_u64_digits() {
            out += i.count_ones() as u64;
        }
        out
    }

    fn count_zeros_64(&self) -> u64 {
        self.bits() - self.count_ones_64()
    }
}

macro_rules! impl_count_bits {
    ($($type:ty),+) => {
        $(
            impl CountBits for $type {
                fn count_ones_64(&self) -> u64 {
                    self.count_ones() as u64
                }

                fn count_zeros_64(&self) -> u64 {
                    self.count_zeros() as u64
                }
            }
        )+
    };
}

impl_count_bits!(
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize
);

/// Failable increment.
pub trait Increment {
    /// Failable increment.
    fn incr(&mut self) -> Option<()>
    where
        Self: Sized;
}

impl<T: One + CheckedAdd> Increment for T {
    /// Failable increment.
    fn incr(&mut self) -> Option<()>
    where
        Self: Sized,
    {
        *self = self.checked_add(&T::one())?;
        Some(())
    }
}

/// Failable decrement.
pub trait Decrement {
    /// Failable decrement.
    fn decr(&mut self) -> Option<()>
    where
        Self: Sized;
}

impl<T: One + CheckedSub> Decrement for T {
    /// Failable decrement.
    fn decr(&mut self) -> Option<()>
    where
        Self: Sized,
    {
        *self = self.checked_sub(&T::one())?;
        Some(())
    }
}

/// Return the specfied number of digits of the Ratio in decimal.
pub trait DigitSequence<T> {
    fn digits(&self, digits: usize) -> Option<String>;
}

impl<T: CheckedAdd + CheckedDiv + CheckedMul + CheckedSub + Clone + Display + Integer>
    DigitSequence<T> for Ratio<T>
{
    /// Return the specfied number of digits of the Ratio in decimal.
    fn digits(&self, digits: usize) -> Option<String> {
        rational_decimal_string(self.clone(), digits)
    }
}
