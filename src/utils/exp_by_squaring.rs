use num::{CheckedMul, Integer};

/// Modular exponentiation by squaring
pub fn pow_mod(n: u64, x: u64, p: u64) -> u64 {
    let mut n = u128::from(n);
    let mut x = u128::from(x);
    let p = u128::from(p);
    let mut ans = 1;
    if x <= 0 {
        return 1;
    }
    loop {
        if x == 1 {
            return ((ans * n) % p) as u64;
        }
        if x & 1 == 0 {
            n = (n * n) % p;
            x >>= 1;
            continue;
        } else {
            ans = (ans * n) % p;
            x -= 1;
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
