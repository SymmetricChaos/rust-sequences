use num::{BigInt, Integer, One, Signed, Zero};
use std::{cell::LazyCell, collections::BTreeMap};

// These primes are sufficient witnessses for all 64 bit values
const WITNESSES_BIG: LazyCell<[BigInt; 12]> =
    LazyCell::new(|| [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37].map(|x| BigInt::from(x)));

pub enum Primality {
    NonPrime,
    Prime,
    ProbablyPrime,
}

// 64-bit primality test
// First checks small prime factors then switches to deterministic Miller-Rabin if less than 2^64-1.
pub fn is_prime(mut n: BigInt) -> Primality {
    if n.is_one() || n.is_zero() {
        return Primality::NonPrime;
    }

    if n.is_negative() {
        n = -n;
    }

    // Check by trial
    for witness in WITNESSES_BIG.iter() {
        if &n == witness {
            return Primality::Prime;
        }
        if (&n % witness).is_zero() {
            return Primality::NonPrime;
        }
    }

    let n_minus = n.clone() - 1;

    // Begin Miller-Rabin
    let mut d: BigInt = &n_minus / 2;
    // Slight optimization for dividing out 2 and counting them
    let r = 1_u64 + d.trailing_zeros().unwrap() as u64;
    d >>= d.trailing_zeros().unwrap();

    let two = BigInt::from(2);

    'outer: for w in WITNESSES_BIG.iter() {
        let mut x = w.modpow(&d, &n);

        if x.is_one() || x == n_minus {
            continue 'outer;
        }
        for _ in 0..(r - 1) {
            x = x.modpow(&two, &n);

            if x == n_minus {
                continue 'outer;
            }
        }
        return Primality::NonPrime;
    }

    if n < BigInt::from(u64::MAX) {
        return Primality::Prime;
    } else {
        // This is not well defined at this point
        // Need to run random Miller-Rabin rounds
        return Primality::ProbablyPrime;
    }
}

// Slightly faster primality check that assumes a number is not divisible by any witness and is not 0 or 1
// This is true in the hybrid factoring algorithm after partial trial division
pub fn is_prime_partial(n: BigInt) -> Primality {
    let n_minus = n.clone() - 1;

    // Begin Miller-Rabin
    let mut d: BigInt = &n_minus / 2;
    // Slight optimization for dividing out 2 and counting them
    let r = 1_u64 + d.trailing_zeros().unwrap() as u64;
    d >>= d.trailing_zeros().unwrap();

    let two = BigInt::from(2);

    'outer: for w in WITNESSES_BIG.iter() {
        let mut x = w.modpow(&d, &n);

        if x.is_one() || x == n_minus {
            continue 'outer;
        }
        for _ in 0..(r - 1) {
            x = x.modpow(&two, &n);

            if x == n_minus {
                continue 'outer;
            }
        }
        return Primality::NonPrime;
    }
    Primality::ProbablyPrime
}

/// Find a factor using Pollard's Rho
fn pollards_rho(n: BigInt) -> Option<BigInt> {
    for s in num::iter::range(BigInt::from(2), &n - 2) {
        let mut x: BigInt = s.clone();
        let mut y: BigInt = s.clone();
        let mut d: BigInt = BigInt::one();
        while d.is_one() {
            x = ((&x * &x) + 1) % &n;
            y = ((&y * &y) + 1) % &n;
            y = ((&y * &y) + 1) % &n;
            d = x.abs_sub(&y).gcd(&n);
        }
        if &d != &n {
            return Some(d);
        }
    }
    None
}

// // Factor out all primes up to 37 and return what is left after checking the remainder is prime
// // Can completely factor any number up 1369
// // This is sufficient to factor ~38% of 32-bit numbers
// pub fn partial_trial_division(mut n: BigInt, map: &mut BTreeMap<BigInt, BigInt>) -> BigInt {
//     for p in WITNESSES_BIG.iter() {
//         if !n.is_positive() {
//             break;
//         }
//         let mut ctr = 0;
//         while (n % p).is_zero() {
//             ctr += 1;
//             n = n / p;
//         }
//         if ctr != 0 {
//             map.insert(BigInt::from(p), BigInt::from(ctr));
//         }
//     }
//     // is_prime_partial(n) will not catch this
//     if n.is_one() {
//         return n.clone();
//     }

//     if is_prime_partial(n) {
//         map.insert(n.clone(), BigInt::one());
//         return BigInt::one();
//     }

//     n.clone()
// }

// // Not practical to factor numbers beyond u32s this way
// /// Each prime factor and its multiplicity. Returns an empty vector for 0 and 1.
// pub fn prime_factorization(mut n: &BigInt) -> Vec<(BigInt, BigInt)> {
//     // Handle 0 and 1
//     if !n.is_positive() {
//         return Vec::new();
//     }

//     // BTreeMap will provide us with easy way reference each prime and its multiplicity, and eventually an ordered output
//     // Doesn't seem to be a large performance difference if using HashMap
//     let mut map = BTreeMap::new();

//     // Handle primes and numbers with small factors. This is sufficient to factor ~38% of 32-bit numbers
//     if partial_trial_division(&n, &mut map).is_one() {
//         return map.into_iter().collect_vec();
//     }

//     // Iteratively use Pollard's Rho
//     let mut factors = vec![n.clone()];
//     while !factors.is_empty() {
//         if let Some(f) = pollards_rho(&factors.pop().unwrap()) {
//             if is_prime_partial(&f.0) {
//                 map.entry(f.0)
//                     .and_modify(|x| *x += 1)
//                     .or_insert(BigInt::one());
//             } else {
//                 factors.push(f.0);
//             }
//             if is_prime_partial(&f.1) {
//                 map.entry(f.1)
//                     .and_modify(|x| *x += 1)
//                     .or_insert(BigInt::one());
//             } else {
//                 factors.push(f.1);
//             }
//         } else {
//             break;
//         }
//     }

//     map.into_iter().collect_vec()
// }

// crate::print_sequences!(

//     prime_factorization(&BigInt::from(363747780)).into_iter(), 10, "{:?}", ", ";

// );
