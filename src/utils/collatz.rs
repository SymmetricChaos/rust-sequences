use num::{CheckedAdd, CheckedMul, Integer};

pub fn collatz<T: Clone + Integer + CheckedAdd + CheckedMul>(n: T) -> Option<T> {
    if n.is_even() {
        Some(n.clone() / (T::one() + T::one()))
    } else {
        n.checked_mul(&(T::one() + T::one() + T::one()))?
            .checked_add(&T::one())
    }
}

pub fn reduced_collatz<T: Clone + Integer + CheckedAdd + CheckedMul>(n: T) -> Option<T> {
    let mut n = n;
    if n.is_odd() {
        n = n
            .checked_mul(&(T::one() + T::one() + T::one()))?
            .checked_add(&T::one())?;
    };
    while n.is_even() {
        n = n.clone() / (T::one() + T::one())
    }
    Some(n)
}
