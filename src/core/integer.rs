use num::{BigInt, Integer, Signed, Zero};

/// The integers in the canonical ordering.
/// 0, 1, -1, 2, -2, 3, -3, 4, -4, 5...
pub struct Integers {
    val: BigInt,
}

impl Integers {
    pub fn new() -> Self {
        Self {
            val: BigInt::from(0),
        }
    }

    pub fn nth<T>(n: T) -> BigInt
    where
        BigInt: From<T>,
    {
        let n = BigInt::from(n);

        if n.is_zero() {
            return n;
        } else if n.is_odd() {
            (n + 1) / 2
        } else {
            -n / 2
        }
    }
}

impl Iterator for Integers {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        if self.val.is_positive() {
            self.val *= BigInt::from(-1);
        } else {
            self.val *= BigInt::from(-1);
            self.val += 1;
        };
        Some(out)
    }
}

crate::check_times!(Integers::new(), 3_500_000);

crate::print_values!(
    Integers::new(), 5, 10;
);

crate::check_sequences!(
    Integers::new(), 0, 10, [0, 1, -1, 2, -2, 3, -3, 4, -4, 5];
);
