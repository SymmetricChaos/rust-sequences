use crate::Number;
use num::{BigInt, CheckedAdd, Integer, One, Zero};

/// The Pell numbers.
///
/// ```text
/// 0, 1, 2, 5, 12, 29, 70, 169, 408, 985, 2378, 5741, 13860, 33461...
/// ```
pub struct Pell<T> {
    a: T,
    b: T,
}

impl Pell<Number> {
    pub fn new() -> Self {
        Self { a: 0, b: 1 }
    }
}

impl Pell<BigInt> {
    pub fn new_big() -> Self {
        Self {
            a: BigInt::zero(),
            b: BigInt::one(),
        }
    }
}

impl<T: Clone + CheckedAdd + Integer> Iterator for Pell<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let t = self.a.checked_add(&self.b.checked_add(&self.b)?)?;
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

/// The companion Pell numbers.
///
/// ```text
/// 2, 2, 6, 14, 34, 82, 198, 478, 1154, 2786, 6726, 16238, 39202...
/// ```
pub struct CompanionPell<T> {
    a: T,
    b: T,
}

impl CompanionPell<Number> {
    pub fn new() -> Self {
        Self { a: 2, b: 2 }
    }
}

impl CompanionPell<BigInt> {
    pub fn new_big() -> Self {
        Self {
            a: BigInt::from(2),
            b: BigInt::from(2),
        }
    }
}

impl<T: Clone + CheckedAdd + Integer> Iterator for CompanionPell<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let t = self.a.checked_add(&self.b.checked_add(&self.b)?)?;
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

crate::check_iteration_times!(
    Pell::new_big(), 90_000;
);

crate::check_sequences!(
    Pell::new_big(), [0_u64, 1, 2, 5, 12, 29, 70, 169, 408, 985, 2378, 5741, 13860, 33461, 80782, 195025, 470832, 1136689, 2744210, 6625109, 15994428, 38613965, 93222358, 225058681, 543339720, 1311738121, 3166815962, 7645370045, 18457556052, 44560482149, 107578520350, 259717522849];
    CompanionPell::new_big(), [2_u64, 2, 6, 14, 34, 82, 198, 478, 1154, 2786, 6726, 16238, 39202, 94642, 228486, 551614, 1331714, 3215042, 7761798, 18738638, 45239074, 109216786, 263672646, 636562078, 1536796802, 3710155682, 8957108166, 21624372014, 52205852194, 126036076402, 304278004998, 734592086398];
);

crate::sample_sequences!(
    Pell::new_big();
    CompanionPell::new_big();
);
