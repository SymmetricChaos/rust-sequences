use num::{BigInt, One, Zero};

/// The bits of the Thue-Morse sequence.
/// 0, 1, 1, 0, 1, 0, 0, 1, 1, 0...
pub struct ThueMorse<T> {
    value: BigInt,
    ctr: u64,
    zero: T,
    one: T,
}

impl<T: One + Zero> ThueMorse<T> {
    pub fn new() -> Self {
        Self {
            value: BigInt::one() + BigInt::one(),
            ctr: 0,
            zero: T::zero(),
            one: T::one(),
        }
    }
}

impl ThueMorse<BigInt> {
    pub fn new_big() -> Self {
        Self {
            value: BigInt::one() + BigInt::one(),
            ctr: 0,
            zero: BigInt::zero(),
            one: BigInt::one(),
        }
    }
}

impl ThueMorse<bool> {
    pub fn new_bool() -> Self {
        Self {
            value: BigInt::one() + BigInt::one(),
            ctr: 0,
            zero: false,
            one: true,
        }
    }
}

impl<T: Clone> Iterator for ThueMorse<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = match self.value.bit(self.ctr) {
            true => Some(self.one.clone()),
            false => Some(self.zero.clone()),
        };
        if self.ctr == self.value.bits() - 1 {
            let n_bits = self.value.bits().next_power_of_two();
            for n in 0..n_bits {
                self.value.set_bit(n_bits + n, !self.value.bit(n));
            }
        };
        self.ctr += 1;
        out
    }
}

crate::check_iteration_times!(
    ThueMorse::<BigInt>::new(), 6_000_000;
    ThueMorse::<i32>::new(), 9_000_000;
    ThueMorse::new_bool(), 9_000_000;
);

crate::check_sequences!(
    ThueMorse::<BigInt>::new(), [0, 1, 1, 0, 1, 0, 0, 1, 1, 0];
);
