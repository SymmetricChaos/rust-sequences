use num::{CheckedAdd, CheckedSub, One};
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
