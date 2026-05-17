use num::{BigInt, BigUint, CheckedAdd, CheckedDiv, CheckedMul, Integer, One};

/// Next value of the Collatz sequence.
pub fn collatz<T: Clone + Integer + CheckedAdd + CheckedDiv + CheckedMul>(n: T) -> Option<T> {
    if n.is_even() {
        n.checked_div(&(T::one() + T::one()))
    } else {
        n.checked_mul(&(T::one() + T::one() + T::one()))?
            .checked_add(&T::one())
    }
}

/// Next odd value of the Collatz sequence.
pub fn reduced_collatz<T: Clone + Integer + CheckedAdd + CheckedDiv + CheckedDiv + CheckedMul>(
    n: T,
) -> Option<T> {
    let mut n = n.clone();
    if n.is_odd() {
        n = n
            .checked_mul(&(T::one() + T::one() + T::one()))?
            .checked_add(&T::one())?;
    };
    while n.is_even() {
        n = n.checked_div(&(T::one() + T::one()))?;
    }
    Some(n)
}

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
                        self.checked_mul(&(<$t>::one() + <$t>::one() + <$t>::one()))?
                            .checked_add(<$t>::one())
                    }
                }

                fn reduced_collatz(&self) -> Option<$t> {
                    let mut n = *self;
                    if self.is_odd() {
                        n = n
                            .checked_mul((<$t>::one() + <$t>::one() + <$t>::one()))?
                            .checked_add(<$t>::one())?;
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
