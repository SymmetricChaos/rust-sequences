use itertools::Itertools;
use num::{CheckedAdd, CheckedMul, Integer};
use std::collections::BTreeMap;

// Modular exponentiation I got from a website
fn modular_exponent<N>(n: N, x: N, p: N) -> u64
where
    u128: From<N>,
{
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

// These primes are sufficient witnessses for all 64 bit values
const WITNESSES_64: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

// 64-bit primality test
// First checks small prime factors then switches to deterministic Miller-Rabin
pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    // Check by trial
    for witness in WITNESSES_64 {
        if n == witness {
            return true;
        }
        if n % witness == 0 {
            return false;
        }
    }

    // Begin Miller-Rabin
    let mut d = (n - 1) / 2;
    // Slight optimization for dividing out 2 and counting them
    let r = 1_u64 + d.trailing_zeros() as u64;
    d >>= d.trailing_zeros();

    'outer: for w in WITNESSES_64.into_iter() {
        let mut x = modular_exponent(w, d, n);

        if x == 1 || x == n - 1 {
            continue 'outer;
        }
        for _ in 0..r - 1 {
            x = modular_exponent(x, 2, n);

            if x == n - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

// Slightly faster primality check that assumes a number is not divisible by any witness and is not 0 or 1
// This is true in the hybrid factoring algorithm after partial trial division
pub fn is_prime_partial(n: u64) -> bool {
    // Begin Miller-Rabin
    let mut d = (n - 1) / 2;
    // Slight optimization for dividing out 2 and counting them
    let r = 1_u64 + d.trailing_zeros() as u64;
    d >>= d.trailing_zeros();

    'outer: for w in WITNESSES_64.into_iter() {
        let mut x = modular_exponent(w, d, n);

        if x == 1 || x == n - 1 {
            continue 'outer;
        }
        for _ in 0..r - 1 {
            x = modular_exponent(x, 2, n);

            if x == n - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

/// Find a factor using Pollard's Rho
fn pollards_rho(n: u64) -> Option<(u64, u64)> {
    let n = u128::from(n);
    for s in 2..(n - 2) {
        let mut x = s;
        let mut y = s;
        let mut d = 1;
        while d == 1 {
            x = ((x * x) + 1) % n;
            y = ((y * y) + 1) % n;
            y = ((y * y) + 1) % n;
            d = x.abs_diff(y).gcd(&n);
        }
        if d != n {
            return Some((d as u64, (n / d) as u64));
        }
    }
    None
}

// Factor out all primes up to 37 and return what is left
// Can completely factor any number up 1369
fn partial_trial_division(mut n: u64, map: &mut BTreeMap<u64, u64>) -> u64 {
    for p in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n <= 1 {
            break;
        }
        let mut ctr = 0;
        while n % p == 0 {
            ctr += 1;
            n = n / p;
        }
        if ctr != 0 {
            map.insert(p, ctr);
        }
    }
    if is_prime(n) {
        map.insert(n, 1);
        n = 1;
    }
    n
}

// Not practical to factor numbers beyond u32s this way
/// Each prime factor and its multiplicity
pub fn prime_factorization(mut n: u64) -> Vec<(u64, u64)> {
    // Handle 0 and 1
    if n <= 1 {
        return Vec::new();
    }

    // Shortcut primes
    if is_prime(n) {
        return vec![(n, 1)];
    }

    // BTreeMap will provide us with easy way reference each prime and its multiplicity, and eventually an ordered output
    // Doesn't seem to be a large performance difference if using HashMap
    let mut map = BTreeMap::new();

    // Handle small factors
    n = partial_trial_division(n, &mut map);
    if n == 1 {
        return map.into_iter().collect_vec();
    }

    // Iteratively use Pollard's Rho
    let mut factors = vec![n];
    while !factors.is_empty() {
        if let Some(f) = pollards_rho(factors.pop().unwrap()) {
            if is_prime_partial(f.0 as u64) {
                map.entry(f.0).and_modify(|x| *x += 1).or_insert(1);
            } else {
                factors.push(f.0);
            }
            if is_prime_partial(f.1 as u64) {
                map.entry(f.1).and_modify(|x| *x += 1).or_insert(1);
            } else {
                factors.push(f.1);
            }
        } else {
            break;
        }
    }

    map.into_iter().collect_vec()
}

/// Factor a number into prime powers
pub fn prime_power_factorization(n: u64) -> Vec<u64> {
    prime_factorization(n)
        .iter()
        .map(|x| x.0.pow(x.1 as u32))
        .collect_vec()
}

/// Number of divisors of n
pub fn number_of_divisors(n: u64) -> u64 {
    let mut out = 1;
    for (_, multiplicity) in prime_factorization(n) {
        out *= multiplicity + 1;
    }
    out
}

/// Sum of divisors of n
/// Defined as 0 for n = 0
pub fn sum_of_divisors(n: u64) -> Option<u64> {
    if n == 0 {
        Some(0)
    } else {
        let v = prime_factorization(n);
        let mut out = 1;
        for (prime, multiplicity) in v {
            let mut s = 1;
            let mut n = prime;
            for _ in 0..multiplicity {
                s = s.checked_add(&n)?;
                n = n.checked_mul(prime)?;
            }
            out = out.checked_mul(&s)?;
        }
        Some(out)
    }
}

/// Aliquot sum n, sum of proper divisors
/// Defined as 0 for n = 0
pub fn aliquot_sum(n: u64) -> Option<u64> {
    match sum_of_divisors(n) {
        Some(total) => Some(total - n),
        None => None,
    }
}

/// Squarefree kernel (radical) of a number, product of unique prime factors, largest squarefree factor
pub fn squarefree_kernel(n: u64) -> u64 {
    prime_factorization(n).iter().fold(1, |acc, p| acc * p.0)
}

/// Euler's totient function. Number of positive integers coprime to n and less than n.
pub fn totient(n: u64) -> u64 {
    prime_factorization(n)
        .iter()
        .fold(1, |acc, x| acc * (x.0.pow((x.1 - 1) as u32) * (x.0 - 1)))
}

/// Euler's cototient function. Number of positive integers not coprime to n and less than n.
pub fn cototient(n: u64) -> u64 {
    n - totient(n)
}

crate::print_values!(
    factors, formatter "{:?}", sep ", ";
    prime_factorization(363747780).into_iter(), 0, 10;
    prime_power_factorization(363747780).into_iter(), 0, 10;
);

#[cfg(test)]
mod tests {

    use std::{io::Write, u32};

    use crate::core::{Composites, Primes};

    use super::*;

    #[test]
    fn is_prime_correctness() {
        for p in Primes::<u64>::new().take(1_000_000) {
            assert!(is_prime(p));
        }
        for c in Composites::<u64>::new().take(1_000_000) {
            assert!(!is_prime(c));
        }
    }

    #[test]
    fn prime_factorization_speed_test() {
        let mut sum = 0;
        let mut longest = (0, 0, vec![]);
        let start = 1;
        std::fs::File::create("src/core/_prime_factorization_speed_test.txt").unwrap();
        for i in start..=(u32::MAX as u64) {
            let t = std::time::Instant::now();
            let fs = prime_factorization(i);
            let d = std::time::Instant::now() - t;

            // Correctness check
            let prod = fs.iter().fold(1, |acc, (pr, ct)| acc * pr.pow(*ct as u32));
            assert_eq!(i, prod);

            let m = d.as_micros();
            if m > longest.0 {
                longest = (m, i, fs)
            }
            sum = sum + &m;

            if i % 500_000 == 0 {
                let mut file = std::fs::File::options()
                    .append(true)
                    .open("src/core/_prime_factorization_speed_test.txt")
                    .unwrap();
                file.write_all(format!("searched range {start}..={i}\n").as_bytes())
                    .unwrap();
                file.write_all(
                    format!(
                        "longest time to factor: {:?}us ({} = {:?})\n",
                        longest.0, longest.1, longest.2
                    )
                    .as_bytes(),
                )
                .unwrap();
                file.write_all(
                    format!(
                        "average time to factor: {:?}us\n\n",
                        sum / (i - start) as u128
                    )
                    .as_bytes(),
                )
                .unwrap();
                file.flush().unwrap();
            }
        }
    }
}
