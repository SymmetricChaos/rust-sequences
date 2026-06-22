use crate::Number;
use num::{CheckedMul, Integer};

/// Modular exponentiation by squaring with unchecked operations.
pub fn pow_mod(n: Number, p: Number, m: Number) -> Number {
    #[cfg(target_pointer_width = "64")]
    let mut n = i128::from(n);
    #[cfg(target_pointer_width = "64")]
    let mut p = i128::from(p);
    #[cfg(target_pointer_width = "64")]
    let m = i128::from(m);

    #[cfg(target_pointer_width = "32")]
    let mut n = i64::from(n);
    #[cfg(target_pointer_width = "32")]
    let mut p = i64::from(p);
    #[cfg(target_pointer_width = "32")]
    let m = i64::from(m);

    let mut ans = 1;

    if p <= 0 {
        return 1;
    }
    loop {
        if p == 1 {
            return ((ans * n) % m) as Number;
        }
        if p & 1 == 0 {
            n = (n * n) % m;
            p >>= 1;
            continue;
        } else {
            ans = (ans * n) % m;
            p -= 1;
        }
    }
}

/// Modular exponentiation by squaring with checked multiplication
pub fn checked_pow_mod<T: Integer + Clone + CheckedMul>(n: T, p: T, m: T) -> Option<T> {
    if p == T::zero() {
        return Some(T::one());
    }
    if p == T::one() {
        return Some(n % m);
    }
    let two = T::one() + T::one();
    if p.clone() % two.clone() == T::zero() {
        return Some(
            checked_pow_mod(n.checked_mul(&n)? % m.clone(), p / two.clone(), m.clone())?
                % m.clone(),
        );
    } else {
        return Some(
            (n.clone()
                * checked_pow_mod(
                    n.checked_mul(&n)? % m.clone(),
                    (p - T::one()) / two,
                    m.clone(),
                )?)
                % m,
        );
    }
}
