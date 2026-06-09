use crate::core::rational_decimal_string;
use num::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Integer, One, rational::Ratio};
use std::fmt::Display;

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
