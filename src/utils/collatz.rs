use num::{CheckedAdd, CheckedMul, Integer};

/// Apply the Collatz function. n/2 if n is even and 3n+1 if n is odd.
pub fn collatz<T: CheckedAdd + CheckedMul + Clone + Integer>(n: T) -> Option<T> {
    if n.is_even() {
        Some(n.clone() / (T::one() + T::one()))
    } else {
        n.checked_mul(&(T::one() + T::one() + T::one()))?
            .checked_add(&T::one())
    }
}

/// Apply the reduced Collatz function. n/2^k if n is even and (3n+1)2^k if n is odd, or the largest value of k that gives an integer. Always produces an odd number.
pub fn reduced_collatz<T: CheckedAdd + CheckedMul + Clone + Integer>(n: T) -> Option<T> {
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
