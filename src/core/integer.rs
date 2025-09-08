use num::{BigInt, PrimInt, Signed, Zero};

/// The integers in the canonical ordering.
/// 0, 1, -1, 2, -2, 3, -3, 4, -4, 5...
pub struct Integer {
    val: BigInt,
}

impl Integer {
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
        }
        // fully qualified name to avoid name collision
        else if num::Integer::is_odd(&n) {
            (n + 1) / 2
        } else {
            -n / 2
        }
    }
}

impl Iterator for Integer {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        if self.val.is_positive() {
            self.val = -&self.val;
        } else {
            self.val = -&self.val;
            self.val += 1;
        };
        Some(out)
    }
}

/// The integers in the canonical ordering.
/// 0, 1, -1, 2, -2, 3, -3, 4, -4, 5...
pub struct IntegerGeneric<T> {
    val: T,
}

impl<T: PrimInt> IntegerGeneric<T> {
    pub fn new() -> Self {
        Self { val: T::zero() }
    }
}

impl<T: PrimInt + Signed> Iterator for IntegerGeneric<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        if self.val.is_positive() {
            self.val = -self.val.clone();
        } else {
            self.val = -self.val.clone();
            self.val = self.val.checked_add(&T::one())?;
        };
        Some(out)
    }
}

crate::check_iteration_times!(Integer::new(), 3_700_000);

crate::print_values!(
    Integer::new(), 5, 10;
);

crate::check_sequences!(
    Integer::new(), 0, 10, [0, 1, -1, 2, -2, 3, -3, 4, -4, 5];
);
