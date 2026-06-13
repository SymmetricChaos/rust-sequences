use num::PrimInt;

use crate::rngs::{SQRTUMAX, UMAX};

/// A linear congruential generator.
///
/// ```text
/// n_{x} = (n_{x-1} * a + c) % m
///
/// n = 47594118, a = 23, b = 0, m = 100000001 (Lehmer's LCG)
/// 47594118, 94664704, 77288171, 77627916, 85442051, 65167154...
///
/// n = 1, a = 65539, b = 0, m = 2147483648 (RANDU LCG)
/// 1, 65539, 393225, 1769499, 7077969, 26542323, 95552217, 334432395...
/// ```
pub struct Lcg<T> {
    n: T,
    a: T,
    c: T,
    m: T,
}

#[cfg(target_pointer_width = "32")]
impl Lcg<u32> {
    /// To prevent overflow during multiplication n, a, and m must all be less than the square root of the maximum value of the type and x must not cause overflow when added to m.
    pub fn new(n: u32, a: u32, c: u32, m: u32) -> Self {
        assert!(n < SQRTUMAX, "n must be less than {SQRTUMAX}");
        assert!(a < SQRTUMAX, "a must be less than {SQRTUMAX}");
        assert!(m < SQRTUMAX, "m must be less than {SQRTUMAX}");
        assert!(c < UMAX - m, "c must be less than {}", UMAX - m);
        Self { n, a, c, m }
    }
}

#[cfg(target_pointer_width = "64")]
impl Lcg<u64> {
    /// To prevent overflow during multiplication n, a, and m must all be less than the square root of the maximum value of the type and x must not cause overflow when added to m.
    pub fn new(n: u64, a: u64, c: u64, m: u64) -> Self {
        assert!(n < SQRTUMAX, "n must be less than {SQRTUMAX}");
        assert!(a < SQRTUMAX, "a must be less than {SQRTUMAX}");
        assert!(m < SQRTUMAX, "m must be less than {SQRTUMAX}");
        assert!(c < UMAX - m, "c must be less than {}", UMAX - m);
        Self { n, a, c, m }
    }
}

impl<T: PrimInt> Iterator for Lcg<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.n;
        self.n = (self.n * self.a) % self.m;
        self.n = (self.n + self.c) % self.m;
        Some(out)
    }
}

crate::check_sequences!(
    // Lehmer's LCG
    Lcg::new(47594118, 23, 0, 100000001), [47594118, 94664704, 77288171, 77627916, 85442051, 65167154, 98844528, 73424122, 88754790, 41360150, 51283441, 79519132, 28940018, 65620408, 9269369, 13195485, 3496152, 80411496, 49464390, 37680959, 66662049, 33227112, 64223569];
    // The infamous RANDU LCG
    Lcg::new(1, 65539, 0, 1<<31), [1, 65539, 393225, 1769499, 7077969, 26542323, 95552217, 334432395, 1146624417, 1722371299, 14608041, 1766175739, 1875647473, 1800754131, 366148473, 1022489195, 692115265, 1392739779, 2127401289, 229749723, 1559239569, 845238963, 1775695897, 899541067, 153401569];
);

crate::sample_sequences!(
    Lcg::new(47594118, 23, 0, 100000001);
    Lcg::new(1, 65539, 0, 1<<31);
);
