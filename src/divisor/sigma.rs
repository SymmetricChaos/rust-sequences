use crate::{core::traits::Increment, utils::divisibility::sigma};

/// Sum of powers of divisors of n. Also known as sigma_z(n) [σ_z(n)].
pub struct Sigma {
    ctr: u64,
    e: u32,
}

impl Sigma {
    /// Only u64 output is supported.
    pub fn new(e: u32) -> Self {
        Self { ctr: 0, e }
    }
}

impl Iterator for Sigma {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.incr()?;
        sigma(self.ctr, self.e)
    }
}
crate::check_sequences!(
    Sigma::new(2), [1, 5, 10, 21, 26, 50, 50, 85, 91, 130, 122, 210, 170, 250, 260, 341, 290, 455, 362, 546, 500, 610, 530, 850, 651, 850, 820, 1050, 842, 1300, 962, 1365, 1220, 1450, 1300, 1911, 1370, 1810, 1700, 2210, 1682, 2500, 1850, 2562, 2366, 2650, 2210, 3410, 2451, 3255];
    Sigma::new(3), [1, 9, 28, 73, 126, 252, 344, 585, 757, 1134, 1332, 2044, 2198, 3096, 3528, 4681, 4914, 6813, 6860, 9198, 9632, 11988, 12168, 16380, 15751, 19782, 20440, 25112, 24390, 31752, 29792, 37449, 37296, 44226, 43344, 55261, 50654, 61740, 61544, 73710, 68922, 86688];
);
