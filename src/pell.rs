use num::{BigInt, CheckedAdd, One, Zero};

/// The Pell numbers.
///
/// 0, 1, 2, 5, 12, 29, 70, 169, 408, 985...
pub struct Pell<T> {
    a: T,
    b: T,
}

impl<T: CheckedAdd + Clone + One + Zero> Pell<T> {
    pub fn new() -> Self {
        Self {
            a: T::zero(),
            b: T::one(),
        }
    }
}

impl Pell<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: CheckedAdd + Clone + One + Zero> Iterator for Pell<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let t = self.a.checked_add(&(self.b.checked_add(&self.b)?))?;
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

/// The companion Pell numbers.
/// 2, 2, 6, 14, 34, 82, 198, 478, 1154, 2786...
pub struct CompanionPell<T> {
    a: T,
    b: T,
}

impl<T: CheckedAdd + Clone + One + Zero> CompanionPell<T> {
    pub fn new() -> Self {
        Self {
            a: T::one() + T::one(),
            b: T::one() + T::one(),
        }
    }
}

impl CompanionPell<BigInt> {
    pub fn new_big() -> Self {
        Self {
            a: BigInt::one() + BigInt::one(),
            b: BigInt::one() + BigInt::one(),
        }
    }
}

impl<T: CheckedAdd + Clone + One + Zero> Iterator for CompanionPell<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let t = self.a.checked_add(&(self.b.checked_add(&self.b)?))?;
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
