use num::{Signed, bigint::Sign};
use std::{array::IntoIter, iter::Cycle, marker::PhantomData};

/// Creates commonly used iterators that alternate between two values.
pub struct Alternating<T> {
    _phantom: PhantomData<T>,
}

impl Alternating<bool> {
    /// Alternate between true and false
    pub fn true_false() -> Cycle<IntoIter<bool, 2>> {
        [true, false].into_iter().cycle()
    }

    /// Alternate between false and true
    pub fn false_true() -> Cycle<IntoIter<bool, 2>> {
        [false, true].into_iter().cycle()
    }
}

impl Alternating<Sign> {
    /// Alternate between plus sign and minus sign
    pub fn plus_minus() -> Cycle<IntoIter<Sign, 2>> {
        [Sign::Plus, Sign::Minus].into_iter().cycle()
    }

    /// Alternate between minus sign and plus sign
    pub fn minus_plus() -> Cycle<IntoIter<Sign, 2>> {
        [Sign::Minus, Sign::Plus].into_iter().cycle()
    }
}

impl<T: Signed + Clone> Alternating<T> {
    /// Alternate between 1 and -1
    pub fn pos_neg() -> Cycle<IntoIter<T, 2>> {
        [T::one(), -T::one()].into_iter().cycle()
    }

    /// Alternate between -1 and 1
    pub fn neg_pos() -> Cycle<IntoIter<T, 2>> {
        [-T::one(), T::one()].into_iter().cycle()
    }
}
