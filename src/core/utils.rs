use std::collections::BTreeMap;

use crate::core::Primes;
use itertools::Itertools;
use num::{Integer, PrimInt};

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

// 64-bit primality test
// First checks small prime factors then switches to deterministic Miller-Rabin
pub fn is_prime<N>(n: N) -> bool
where
    u64: From<N>,
{
    let n = u64::from(n);

    if n <= 1 {
        return false;
    }

    // These primes are sufficient witnessses
    let witnesses = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

    // Check by trial
    for witness in witnesses {
        if n == witness {
            return true;
        }
        if n % witness == 0 {
            return false;
        }
    }

    // Begin Miller-Rabin
    let mut d = (n - 1) / 2;
    let mut r = 1_u64;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }

    'outer: for w in witnesses.into_iter() {
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
fn pollards_rho<N: PrimInt>(n: N, s: N) -> Option<u32>
where
    u64: From<N>,
{
    let n = u64::from(n);
    let mut x = u64::from(s);
    let mut y = u64::from(s);
    let mut d = 1;
    while d == 1 {
        x = ((x * x) + 1) % n;
        y = ((y * y) + 1) % n;
        y = ((y * y) + 1) % n;
        d = x.abs_diff(y).gcd(&n);
    }
    if d == n { None } else { Some(d as u32) }
}

// Factor out all primes up to 19 and returns what is left
fn partial_trial_division(mut n: u32, map: &mut BTreeMap<u32, u32>) -> u32 {
    for p in [2, 3, 5, 7, 11, 13, 17, 19] {
        if n <= 1 {
            break;
        }
        if is_prime(n) {
            map.entry(n).and_modify(|x| *x += 1).or_insert(1);
            n = 1;
            break;
        }
        let mut ctr = 0;
        while n % p == 0 {
            ctr += 1;
            n = n / p;
        }
        if ctr != 0 {
            map.entry(p).and_modify(|x| *x += ctr).or_insert(ctr);
        }
    }
    n
}

fn trial_division(mut n: u32) -> Vec<(u32, u32)> {
    let mut out = Vec::new();
    for p in Primes::new() {
        if n <= 1 {
            break;
        }
        if is_prime(n) {
            out.push((n, 1));
            break;
        }
        let mut ctr = 0;
        while n % p == 0 {
            ctr += 1;
            n = n / p;
        }
        if ctr != 0 {
            out.push((p, ctr));
        }
    }
    out
}

macro_rules! apply_trial_division {
    ($n:ident, $map:ident) => {
        if is_prime($n) {
            $map.entry($n).and_modify(|x| *x += 1).or_insert(1);
        } else {
            for (pr, ct) in trial_division($n) {
                $map.entry(pr).and_modify(|x| *x += ct).or_insert(ct);
            }
        }
    };
}

// Not practical to factor numbers beyond u32s this way, even large u32s are generally not possible
/// Each prime factor and its multiplicity
pub fn prime_factorization(mut n: u32) -> Vec<(u32, u32)> {
    // Handle 0 and 1
    if n <= 1 {
        return Vec::new();
    }
    // Shortcut primes
    if is_prime(n) {
        return vec![(n, 1)];
    }

    // BTreeMap will provide us with easy way reference each prime and its multiplicity, and eventually an ordered output
    let mut map = BTreeMap::new();

    // Handle small factors
    n = partial_trial_division(n, &mut map);
    if is_prime(n) {
        map.entry(n).and_modify(|x| *x += 1).or_insert(1);
        return map.into_iter().collect_vec();
    } else if n == 1 {
        return map.into_iter().collect_vec();
    }

    // Try every value for pollard's rho method to split into two factors then use trial division
    for s in 2..n - 2 {
        if let Some(p) = pollards_rho(n, s) {
            apply_trial_division!(p, map);
            let q = n / p;
            apply_trial_division!(q, map);
            return map.into_iter().collect_vec();
        }
    }

    // apply_trial_division!(n, map);

    map.into_iter().collect_vec()
}

/// Factor a number into prime powers
pub fn prime_power_factorization(mut n: u32) -> Vec<u32> {
    // Shortcut primes
    if is_prime(n) {
        return vec![n];
    }
    let mut out = Vec::new();
    for p in Primes::<u32>::new() {
        if n <= 1 {
            break;
        }
        let mut factor = 1;
        while n % p == 0 {
            factor *= p;
            n = n / p;
        }
        if factor != 1 {
            out.push(factor);
        }
    }
    out
}

pub fn number_of_divisors(n: u32) -> u32 {
    let mut out = 1;
    for (_, multiplicity) in prime_factorization(n) {
        out *= multiplicity + 1;
    }
    out
}

crate::print_values!(
    factors, formatter "{:?}", sep ", ";
    prime_factorization(363747780).into_iter(), 0, 10;
    prime_power_factorization(363747780).into_iter(), 0, 10;
);

#[cfg(test)]
mod tests {

    use std::{io::Write, u32};

    use super::*;

    #[test]
    fn count_div() {
        println!("{}", number_of_divisors(363747780));
    }

    #[test]
    fn prime_factorization_speed_test() {
        let mut sum = 0;
        let mut longest = (0, 0, vec![]);
        std::fs::File::create("src/core/_prime_factorization_speed_test.txt").unwrap();
        for i in 1..=u32::MAX {
            let t = std::time::Instant::now();
            let fs = prime_factorization(i);
            let d = std::time::Instant::now() - t;

            // Correctness check
            let prod = fs.iter().fold(1, |acc, (pr, ct)| acc * pr.pow(*ct));
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
                file.write_all(format!("searched range 1..={i}\n").as_bytes())
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
                    format!("average time to factor: {:?}us\n\n\n", sum / i as u128).as_bytes(),
                )
                .unwrap();
                file.flush().unwrap();
            }
        }
    }
}
