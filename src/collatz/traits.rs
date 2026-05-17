use num::{BigInt, BigUint, Integer};

pub trait CollatzTrait {
    /// Next value of the Collatz sequence.
    fn collatz(&self) -> Option<Self>
    where
        Self: Sized;
    /// Next odd value of the Collatz sequence.
    fn reduced_collatz(&self) -> Option<Self>
    where
        Self: Sized;
}

macro_rules! impl_collatz_trait {
    ($($t:ty),+) => {
        $(
            impl CollatzTrait for $t {
                fn collatz(&self) -> Option<Self> {
                    if self.is_even() {
                        Some(self >> 1)
                    } else {
                        self.checked_mul(3)?
                            .checked_add(1)
                    }
                }

                fn reduced_collatz(&self) -> Option<$t> {
                    let mut n = *self;
                    if self.is_odd() {
                        n = n
                            .checked_mul(3)?
                            .checked_add(1)?;
                    };
                    n = n >> n.trailing_zeros();
                    Some(n)
                }
            }
        )+
    };
}

impl_collatz_trait!(
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize
);

impl CollatzTrait for BigInt {
    fn collatz(&self) -> Option<Self> {
        let n = self.clone();
        if self.is_even() {
            Some(n / 2)
        } else {
            Some(n * 3 + 1)
        }
    }

    fn reduced_collatz(&self) -> Option<Self> {
        let mut n = self.clone();
        if self.is_odd() {
            n = n * 3 + 1;
        };
        while n.is_even() {
            n = n / 2;
        }
        Some(n)
    }
}

impl CollatzTrait for BigUint {
    fn collatz(&self) -> Option<Self> {
        let n = self.clone();
        if self.is_even() {
            Some(n / 2_u32)
        } else {
            Some(n * 3_u32 + 1_u32)
        }
    }

    fn reduced_collatz(&self) -> Option<Self> {
        let mut n = self.clone();
        if self.is_odd() {
            n = n * 3_u32 + 1_u32;
        };
        while n.is_even() {
            n = n / 2_u32;
        }
        Some(n)
    }
}
