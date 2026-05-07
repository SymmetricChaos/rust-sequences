use crate::core::rational_decimal_string;
use num::{BigInt, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Integer, One, rational::Ratio};
use std::fmt::Display;

pub trait CustomNumTraits: CountBits + Decrement + Increment {}

impl<T> CustomNumTraits for T where T: CountBits + Decrement + Increment {}

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

impl<T> Increment for T
where
    T: One + CheckedAdd,
{
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

impl<T> Decrement for T
where
    T: One + CheckedSub,
{
    /// Failable decrement.
    fn decr(&mut self) -> Option<()>
    where
        Self: Sized,
    {
        *self = self.checked_sub(&T::one())?;
        Some(())
    }
}

pub trait DigitSequence<T> {
    fn digits(&self, digits: usize) -> Option<String>;
}

impl<T: CheckedAdd + CheckedDiv + CheckedMul + CheckedSub + Clone + Display + Integer>
    DigitSequence<T> for Ratio<T>
{
    fn digits(&self, digits: usize) -> Option<String> {
        rational_decimal_string(self.clone(), digits)
    }
}
