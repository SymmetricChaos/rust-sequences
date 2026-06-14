use crate::rngs::{HALFUMAX, SQRTUMAX};
use num::PrimInt;

/// Lagged Fibonacci Generator using addition.
///
/// ```text
/// n_{x} = (n_{x-1} + n_{x-2}) % m
///
/// a = 123, b = 456, m = 789
/// 123, 456, 579, 246, 36, 282, 318, 600, 129, 729, 69, 9, 78, 87, 165...
/// ```
pub struct Lfg<T> {
    a: T,
    b: T,
    m: T,
}

#[cfg(target_pointer_width = "32")]
impl Lfg<u32> {
    /// To prevent overflow during addition a, b, and m must all be less than half the maximum value of the type.
    pub fn new(a: u32, b: u32, m: u32) -> Self {
        assert!(a < HALFMAX, "a must be less than {HALFMAX}");
        assert!(b < HALFMAX, "b must be less than {HALFMAX}");
        assert!(m < HALFMAX, "m must be less than {HALFMAX}");
        Self { a, b, m }
    }
}

#[cfg(target_pointer_width = "64")]
impl Lfg<u64> {
    /// To prevent overflow during addition a, b, and m must all be less than half the maximum value of the type.
    pub fn new(a: u64, b: u64, m: u64) -> Self {
        assert!(a < HALFUMAX, "a must be less than {HALFUMAX}");
        assert!(b < HALFUMAX, "b must be less than {HALFUMAX}");
        assert!(m < HALFUMAX, "m must be less than {HALFUMAX}");
        Self { a, b, m }
    }
}

impl<T: PrimInt> Iterator for Lfg<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a;
        let t = (self.a + self.b) % self.m;
        self.a = self.b;
        self.b = t;
        Some(out)
    }
}

/// Lagged Fibonacci Generator using multiplication.
///
/// ```text
/// n_{x} = (n_{x-1} * n_{x-2}) % m
///
/// a = 123, b = 456, m = 789
/// 123, 456, 69, 693, 477, 759, 681, 84, 396, 126, 189, 144, 390, 141...
/// ```
pub struct LfgMult<T> {
    a: T,
    b: T,
    m: T,
}

#[cfg(target_pointer_width = "32")]
impl LfgMult<u32> {
    /// To prevent overflow during multiplication a, b, and m must all be less than the square root of the maximum value of the type.
    pub fn new(a: u32, b: u32, m: u32) -> Self {
        assert!(a < SQRTUMAX, "a must be less than {SQRTUMAX}");
        assert!(b < SQRTUMAX, "b must be less than {SQRTUMAX}");
        assert!(m < SQRTUMAX, "m must be less than {SQRTUMAX}");
        Self { a, b, m }
    }
}

#[cfg(target_pointer_width = "64")]
impl LfgMult<u64> {
    /// To prevent overflow during multiplication a, b, and m must all be less than the square root of the maximum value of the type.
    pub fn new(a: u64, b: u64, m: u64) -> Self {
        assert!(a < SQRTUMAX, "a must be less than {SQRTUMAX}");
        assert!(b < SQRTUMAX, "b must be less than {SQRTUMAX}");
        assert!(m < SQRTUMAX, "m must be less than {SQRTUMAX}");
        Self { a, b, m }
    }
}

impl<T: PrimInt> Iterator for LfgMult<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a;
        let t = (self.a * self.b) % self.m;
        self.a = self.b;
        self.b = t;
        Some(out)
    }
}

crate::sample_sequences!(
    Lfg::new(123, 456, 789);
    LfgMult::new(123, 456, 789);
);
