use num::{BigInt, CheckedAdd, CheckedSub, One};

pub trait CustomNumTraits: Increment + Decrement + CountBits {}

impl<T> CustomNumTraits for T where T: CountBits + Increment + Decrement {}

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
