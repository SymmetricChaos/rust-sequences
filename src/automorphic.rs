use num::{BigInt, One};

pub struct Automorphic {
    value: BigInt,
    digits: u32,
    base: BigInt,
}

impl Automorphic {
    /// Panics if value is greater than the base.
    /// Panics if the value is not a product of factors of the base.
    pub fn new_big<T>(value: T, base: T) -> Self
    where
        BigInt: From<T>,
    {
        let value = BigInt::from(value);
        let base = BigInt::from(base);
        assert!(base > BigInt::one());
        assert!(
            &value < &base,
            "the starting value must be less than the base"
        );
        assert_eq!(
            value,
            (&value * &value) % &base,
            "this value cannot be the start of an automorphic number in this base"
        );
        Self {
            value,
            digits: 1,
            base,
        }
    }
}

impl Iterator for Automorphic {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.value.clone();

        loop {
            self.digits += 1;
            for digit in num::iter::range(BigInt::one(), self.base.clone()) {
                let test: BigInt = &self.value + self.base.pow(self.digits - 1) * digit;
                if (&test * &test) % self.base.pow(self.digits) == test && test != out {
                    self.value = test.clone();
                    return Some(out);
                }
            }
        }
    }
}

/// The sequence of digits that define a k-adic automorphic number, if they exist. That is a number which when squared has all the same digits.
pub struct AutomorphicDigits {
    value: BigInt,
    digits: u32,
    base: BigInt,
}

impl AutomorphicDigits {
    /// Panics if value is greater than the base.
    /// Panics if the value is not a product of factors of the base.
    pub fn new_big<T>(value: T, base: T) -> Self
    where
        BigInt: From<T>,
    {
        let value = BigInt::from(value);
        let base = BigInt::from(base);
        assert!(base > BigInt::one());
        assert!(
            &value < &base,
            "the starting value must be less than the base"
        );
        assert_eq!(
            value,
            (&value * &value) % &base,
            "this value cannot be the start of an automorphic number in this base"
        );
        Self {
            value,
            digits: 1,
            base,
        }
    }
}

impl Iterator for AutomorphicDigits {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.value.clone() / self.base.pow(self.digits - 1);

        self.digits += 1;
        for digit in num::iter::range(BigInt::one(), self.base.clone()) {
            // no need to test 0
            let test: BigInt = &self.value + self.base.pow(self.digits - 1) * digit;
            if (&test * &test) % self.base.pow(self.digits) == test && test != out {
                self.value = test.clone();
            }
        }

        Some(out)
    }
}

crate::check_sequences!(
    Automorphic::new_big(5, 10), 0, 8, [5, 25, 625, 90625, 890625, 2890625, 12890625, 212890625];
    Automorphic::new_big(6, 10), 0, 8, [6, 76, 376, 9376, 109376, 7109376, 87109376, 787109376];
    AutomorphicDigits::new_big(5, 10), 0, 10, [5, 2, 6, 0, 9, 8, 2, 1, 2, 8];
    AutomorphicDigits::new_big(6, 10), 0, 10, [6, 7, 3, 9, 0, 1, 7, 8, 7, 1];
    AutomorphicDigits::new_big(3, 6), 0, 10, [3, 1, 2, 0, 5, 3, 1, 2, 2, 2];
);
