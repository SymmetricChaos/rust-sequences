use crate::utils::miller_rabin::{MR_WITNESSES_U64, miller_rabin};
use itertools::Itertools;
use num::{CheckedAdd, CheckedMul, Integer, rational::Ratio};
use std::collections::BTreeMap;

// First checks small prime factors then switches to deterministic Miller-Rabin.
/// 64-bit primality test
pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    // Check by trial
    for witness in MR_WITNESSES_U64 {
        if n == witness {
            return true;
        }
        if n % witness == 0 {
            return false;
        }
    }

    match miller_rabin(n) {
        super::miller_rabin::MRTest::Prime => true,
        super::miller_rabin::MRTest::Composite(_) => false,
    }
}

// Slightly faster primality check that assumes a number is not divisible by any witness and is not 0 or 1
// This is true in the hybrid factoring algorithm after partial trial division
pub fn is_prime_partial(n: u64) -> bool {
    match miller_rabin(n) {
        super::miller_rabin::MRTest::Prime => true,
        super::miller_rabin::MRTest::Composite(_) => false,
    }
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

// Factor out all primes up to 37 and return what is left after checking the remainder is prime
// Can completely factor any number up 1369
// This is sufficient to factor ~38% of 32-bit numbers
pub fn partial_trial_division(mut n: u64, map: &mut BTreeMap<u64, u64>) -> u64 {
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

    // miller_rabin(n) will not catch this
    if n == 1 {
        return n;
    }

    match miller_rabin(n) {
        super::miller_rabin::MRTest::Prime => {
            map.insert(n, 1);
            n = 1;
        }
        super::miller_rabin::MRTest::Composite(w) => match w {
            Some(d) => {
                let mut ctr = 0;
                while n % d == 0 {
                    ctr += 1;
                    n = n / d;
                }
                map.insert(d, ctr);
            }
            None => (),
        },
    }

    n
}

// Not practical to factor numbers beyond u32s this way
/// Each prime factor and its multiplicity. Returns an empty vector for 0 and 1.
pub fn prime_factorization(mut n: u64) -> Vec<(u64, u64)> {
    // Handle 0 and 1
    if n <= 1 {
        return Vec::new();
    }

    // BTreeMap will provide us with easy way reference each prime and its multiplicity, and eventually an ordered output
    // Doesn't seem to be a large performance difference if using HashMap
    let mut map = BTreeMap::new();

    // Handle numbers with easy to find divisors.
    // This catches anything divisible by a small prime and anything with a factor that the Miller-Rabin test can find.
    n = partial_trial_division(n, &mut map);
    if n == 1 {
        return map.into_iter().collect_vec();
    }
    if is_prime_partial(n) {
        map.entry(n).and_modify(|x| *x += 1).or_insert(1);
        return map.into_iter().collect_vec();
    }

    // Iteratively use Pollard's Rho
    let mut factors = vec![n];
    while !factors.is_empty() {
        let d = factors.pop().unwrap();
        match miller_rabin(d) {
            super::miller_rabin::MRTest::Prime => {
                map.entry(d).and_modify(|x| *x += 1).or_insert(1);
                continue;
            }
            super::miller_rabin::MRTest::Composite(w) => match w {
                Some(x) => {
                    factors.push(x);
                    factors.push(d / x);
                    continue;
                }
                None => (),
            },
        }
        if let Some(f) = pollards_rho(d) {
            factors.push(f.0);
            factors.push(f.1);
        } else {
            break;
        }
    }

    map.into_iter().collect_vec()
}

/// The number of prime factors of n counted with multiplicity.
pub fn big_omega(n: u64) -> u64 {
    prime_factorization(n).into_iter().map(|(_, m)| m).sum()
}

/// The number of distinct prime factors of n.
pub fn small_omega(n: u64) -> u64 {
    prime_factorization(n).into_iter().map(|_| 1).sum()
}

/// Powers of the prime factors of n in descending order
/// returns [] for both 0 and 1
pub fn prime_signature(n: u64) -> Vec<u64> {
    prime_factorization(n)
        .iter()
        .map(|x| x.1)
        .sorted()
        .rev()
        .collect_vec()
}

/// Factor a number into prime powers
pub fn prime_power_factorization(n: u64) -> Vec<u64> {
    prime_factorization(n)
        .iter()
        .map(|x| x.0.pow(x.1 as u32))
        .collect_vec()
}

/// Number of divisors of n. Also known as σ_0(n).
/// Defined as 0 for n = 0.
pub fn number_of_divisors(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut out = 1;
    for (_, multiplicity) in prime_factorization(n) {
        out *= multiplicity + 1;
    }
    out
}

/// Sum of all divisors of n. Also known as σ_1(n).
/// Defined as 0 for n = 0.
/// Returns None if overflow occurs.
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

/// Sum of all divisors of n. Divided by n. Also known as σ_-1(n).
/// Returns None for n = 0 or if overflow occurs.
pub fn abundancy_index(n: u64) -> Option<Ratio<u64>> {
    if n == 0 {
        None
    } else {
        Some(Ratio::new(sum_of_divisors(n)?, n))
    }
}

/// Aliquot sum of n. The sum of all divisors except n itself.
/// Defined as 0 for n = 0.
/// Returns None if overflow occurs.
pub fn aliquot_sum(n: u64) -> Option<u64> {
    match sum_of_divisors(n) {
        Some(total) => Some(total - n),
        None => None,
    }
}

/// The radical of a number, the product of its unique prime factors. Also known as the squarefree kernel or the largest squarefree divisor.
/// Defined as 1 for n == 0.
pub fn radical(n: u64) -> u64 {
    prime_factorization(n).iter().fold(1, |acc, p| acc * p.0)
}

/// Euler's totient function. Number of positive integers coprime to n and less than n.
/// Defined as 0 for n == 0.
pub fn totient(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    prime_factorization(n)
        .iter()
        .fold(1, |acc, x| acc * (x.0.pow((x.1 - 1) as u32) * (x.0 - 1)))
}

/// Euler's cototient function. Number of positive integers not coprime to n and less than n.
/// Defined as 0 for n == 0.
pub fn cototient(n: u64) -> u64 {
    n - totient(n)
}

/// All of the divisors of n.
/// Defined as [0] for n = 0.
pub fn divisors(n: u64) -> Vec<u64> {
    if n == 0 {
        return vec![0];
    }

    if n == 1 {
        return vec![1];
    }

    let mut out = vec![1, n];

    if is_prime(n) {
        return out;
    }

    for f in 2..=n.isqrt() {
        let (d, r) = n.div_rem(&f);
        if r == 0 {
            out.push(f);
            out.push(d);
        }
    }

    out.sort();

    out
}

/// All of the divisors of n except itself.
/// Defined as [] for n = 0.
pub fn proper_divisors(n: u64) -> Vec<u64> {
    if n == 0 || n == 1 {
        return vec![];
    }

    let mut out = vec![1];

    if is_prime(n) {
        return out;
    }

    for f in 2..=n.isqrt() {
        let (d, r) = n.div_rem(&f);
        if r == 0 {
            out.push(f);
            out.push(d);
        }
    }

    out.sort();

    out
}

crate::print_sequences!(
    divisors(2*2*5*7).into_iter(), 20, "{:?}", ", ";
    prime_factorization(363747780).into_iter(), 10, "{:?}", ", ";
    prime_power_factorization(363747780).into_iter(), 10, "{:?}", ", ";
);
