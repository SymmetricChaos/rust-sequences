use crate::factorial::Factorial;
use num::{BigInt, One, Zero};

/// Liouville's constant, decimal expansion of the first number proven to be transcendental. Also the characteristic function of the factorials.
///
/// ```text
/// 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0...
/// ```
pub struct Liouville {
    factorial: Factorial<BigInt>,
    next_factorial: BigInt,
    ctr: BigInt,
}

impl Liouville {
    pub fn new() -> Self {
        let mut factorial = Factorial::new_big();
        factorial.next();
        let next_factorial = factorial.next().unwrap();
        Self {
            factorial,
            next_factorial,
            ctr: BigInt::zero(),
        }
    }
}

impl Iterator for Liouville {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr += 1;
        if self.ctr == self.next_factorial {
            self.next_factorial = self.factorial.next()?;
            return Some(BigInt::one());
        }
        Some(BigInt::zero())
    }
}

crate::check_sequences!(
    Liouville::new(), [1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
);

crate::sample_sequences!(
    Liouville::new();
);
