use num::{Integer, Zero};

use crate::Number;

/// Apply the Collatz function. n/2 if n is even and 3n+1 if n is odd.
pub fn collatz(n: Number) -> Option<Number> {
    if n.is_even() {
        Some(n / 2)
    } else {
        n.checked_mul(3)?.checked_add(1)
    }
}

/// Apply the reduced Collatz function. n/2^k if n is even and (3n+1)/2^k if n is odd, for the largest value of k that gives an integer. Always produces an odd number.
pub fn reduced_collatz(mut n: Number) -> Option<Number> {
    if n.is_zero() {
        return Some(0);
    }
    if n.is_odd() {
        n = n.checked_mul(3)?.checked_add(1)?;
    };
    while n.is_even() {
        n = n / 2;
    }
    Some(n)
}
