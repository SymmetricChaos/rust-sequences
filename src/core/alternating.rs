use num::{Signed, bigint::Sign};
use std::marker::PhantomData;

/// Creates commonly used iterators that alternate between two values.
pub struct Alternating<T> {
    _phantom: PhantomData<T>,
}

impl Alternating<bool> {
    /// Alternate between true and false.
    pub fn true_false() -> impl Iterator<Item = bool> {
        [true, false].into_iter().cycle()
    }

    /// Alternate between false and true.
    pub fn false_true() -> impl Iterator<Item = bool> {
        [false, true].into_iter().cycle()
    }
}

impl Alternating<Sign> {
    /// Alternate between plus sign and minus sign.
    pub fn plus_minus() -> impl Iterator<Item = Sign> {
        [Sign::Plus, Sign::Minus].into_iter().cycle()
    }

    /// Alternate between minus sign and plus sign.
    pub fn minus_plus() -> impl Iterator<Item = Sign> {
        [Sign::Minus, Sign::Plus].into_iter().cycle()
    }
}

impl<T: Signed + Clone> Alternating<T> {
    /// Alternate between 1 and -1.
    pub fn pos_neg() -> impl Iterator<Item = T> {
        [T::one(), -T::one()].into_iter().cycle()
    }

    /// Alternate between -1 and 1.
    pub fn neg_pos() -> impl Iterator<Item = T> {
        [-T::one(), T::one()].into_iter().cycle()
    }
}
