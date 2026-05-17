use num::{CheckedAdd, CheckedDiv, CheckedMul, Integer};

/// Next value of the Collatz function.
pub fn collatz<T: Clone + Integer + CheckedAdd + CheckedDiv + CheckedMul>(n: T) -> Option<T> {
    if n.is_even() {
        n.checked_div(&(T::one() + T::one()))
    } else {
        n.checked_mul(&(T::one() + T::one() + T::one()))?
            .checked_add(&T::one())
    }
}

/// Next odd value of the Collatz function.
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

pub trait Collatz {}
