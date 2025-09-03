use num::{BigInt, Integer, One};

/// The regular numbers, fhose for which the only prime divisors are less than n.
pub struct Smooth {}

impl Smooth {
    pub fn new() -> Self {
        Self {}
    }
}

impl Iterator for Smooth {
    type Item = BigInt;

    fn next(&mut self) -> Option<BigInt> {
        todo!()
    }
}

/// The regular numbers, those for which only of 2, 3, and 5 are prime divisors.
/// 1, 2, 3, 4, 5, 6, 8, 9, 10, 12, 15, 16, 18, 20, 24, 25, 27, 30,
pub struct Regular {
    ctr: BigInt,
}

impl Regular {
    pub fn new() -> Self {
        Self {
            ctr: BigInt::from(0),
        }
    }
}

impl Iterator for Regular {
    type Item = BigInt;

    fn next(&mut self) -> Option<BigInt> {
        loop {
            self.ctr += 1;
            let mut n = self.ctr.clone();
            while n.is_even() {
                n = n / BigInt::from(2);
            }
            while n.is_multiple_of(&BigInt::from(3)) {
                n = n / BigInt::from(3);
            }
            while n.is_multiple_of(&BigInt::from(5)) {
                n = n / BigInt::from(5);
            }
            if n.is_one() {
                return Some(self.ctr.clone());
            }
        }
    }
}

crate::check_time!(
    Regular::new(), 500;
);

crate::check_sequences!(
    Regular::new(), 9, 10, [12, 15, 16, 18, 20, 24, 25, 27, 30, 32];
);
