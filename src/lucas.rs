use num::BigInt;

/// The Lucas numbers.
/// 2, 1, 3, 4, 7, 11, 18, 29, 47, 76...
pub struct Lucas {
    a: BigInt,
    b: BigInt,
}

impl Lucas {
    pub fn new_big() -> Self {
        Self {
            a: BigInt::from(2),
            b: BigInt::from(1),
        }
    }
}

impl Iterator for Lucas {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let t = &self.a + &self.b;
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

crate::check_iteration_times!(
    Lucas::new_big(), 157_500;
);

crate::check_sequences!(
    Lucas::new_big(), 0, 10, [2, 1, 3, 4, 7, 11, 18, 29, 47, 76];
);
