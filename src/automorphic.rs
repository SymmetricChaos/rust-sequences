use num::BigInt;

const BASE: u32 = 10;

pub struct Automorphic {
    value: BigInt,
    digits: u32,
}

impl Automorphic {
    pub fn new_big_5() -> Self {
        Self {
            value: BigInt::from(5),
            digits: 1,
        }
    }

    pub fn new_big_6() -> Self {
        Self {
            value: BigInt::from(6),
            digits: 1,
        }
    }
}

impl Iterator for Automorphic {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.value.clone();

        loop {
            self.digits += 1;
            for digit in 1..=9 {
                let test: BigInt = &self.value + BigInt::from(BASE).pow(self.digits - 1) * digit;
                if (&test * &test) % BigInt::from(BASE).pow(self.digits) == test && test != out {
                    self.value = test.clone();
                    return Some(out);
                }
            }
        }
    }
}

pub struct AutomorphicDigits {
    value: BigInt,
    digits: u32,
}

impl AutomorphicDigits {
    pub fn new_big_5() -> Self {
        Self {
            value: BigInt::from(5),
            digits: 1,
        }
    }

    pub fn new_big_6() -> Self {
        Self {
            value: BigInt::from(6),
            digits: 1,
        }
    }
}

impl Iterator for AutomorphicDigits {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.value.clone() / BigInt::from(BASE).pow(self.digits - 1);

        self.digits += 1;
        for digit in 1..=9 {
            let test: BigInt = &self.value + BigInt::from(BASE).pow(self.digits - 1) * digit;
            if (&test * &test) % BigInt::from(BASE).pow(self.digits) == test && test != out {
                self.value = test.clone();
            }
        }

        Some(out)
    }
}

crate::check_sequences!(
    Automorphic::new_big_5(), 0, 8, [5, 25, 625, 90625, 890625, 2890625, 12890625, 212890625];
    Automorphic::new_big_6(), 0, 8, [6, 76, 376, 9376, 109376, 7109376, 87109376, 787109376];
    AutomorphicDigits::new_big_5(), 0, 10, [5, 2, 6, 0, 9, 8, 2, 1, 2, 8];
    AutomorphicDigits::new_big_6(), 0, 10, [6, 7, 3, 9, 0, 1, 7, 8, 7, 1];
);
