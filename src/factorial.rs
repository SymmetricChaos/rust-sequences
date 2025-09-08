use num::BigInt;

/// The factorial numbers.
/// 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800...
pub struct Factorial {
    val: BigInt,
    ctr: BigInt,
}

impl Factorial {
    pub fn new() -> Self {
        Self {
            val: BigInt::from(1),
            ctr: BigInt::from(2),
        }
    }
}

impl Iterator for Factorial {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val *= &self.ctr;
        self.ctr += 1;
        Some(out)
    }
}

crate::check_iteration_times!(
    Factorial::new(), 37_000;
);

crate::check_sequences!(
    Factorial::new(), 0, 10, [1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800];
);
