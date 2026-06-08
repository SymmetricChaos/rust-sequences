use num::{BigInt, One, Zero};

use crate::Number;

/// The Thue-Morse sequence, the unique fixed point of the mapping 0 -> 01 and 1 -> 10.
///
/// ```text
/// 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1...
/// ```
pub struct ThueMorse<T> {
    value: BigInt,
    ctr: u64,
    zero: T,
    one: T,
}

impl ThueMorse<Number> {
    pub fn new() -> Self {
        Self {
            value: BigInt::from(2),
            ctr: 0,
            zero: 0,
            one: 1,
        }
    }
}

#[cfg(feature = "big_int")]
impl ThueMorse<BigInt> {
    pub fn new_big() -> Self {
        Self {
            value: BigInt::from(2),
            ctr: 0,
            zero: BigInt::zero(),
            one: BigInt::one(),
        }
    }
}

impl<T: Clone> Iterator for ThueMorse<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = match self.value.bit(self.ctr) {
            true => self.one.clone(),
            false => self.zero.clone(),
        };
        if self.ctr == self.value.bits() - 1 {
            let n_bits = self.value.bits().next_power_of_two();
            for n in 0..n_bits {
                self.value.set_bit(n_bits + n, !self.value.bit(n));
            }
        };
        self.ctr += 1;
        Some(out)
    }
}

crate::check_iteration_times!(
    ThueMorse::new_big(), 20_000_000;
    ThueMorse::new(), 20_000_000;
);

crate::check_sequences!(
    ThueMorse::new(), [
        0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0,
        1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1,
        1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1,
        0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0];
);

crate::sample_sequences!(
    ThueMorse::new();
);
