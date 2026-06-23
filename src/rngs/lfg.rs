use crate::rngs::{HALFUMAX, SQRTUMAX, UNumber};
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

impl Lfg<UNumber> {
    /// To prevent overflow during addition a, b, and m must all be less than half the maximum value of the type.
    pub fn new(a: UNumber, b: UNumber, m: UNumber) -> Self {
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

impl LfgMult<UNumber> {
    /// To prevent overflow during multiplication a, b, and m must all be less than the square root of the maximum value of the type.
    pub fn new(a: UNumber, b: UNumber, m: UNumber) -> Self {
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
