use crate::utils::miller_rabin::{is_prime, is_prime_partial, miller_rabin};
use itertools::Itertools;
use num::{CheckedAdd, CheckedMul, Integer, rational::Ratio};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::BTreeMap;

/// Find a factor using Pollard's Rho, switching a parallelized version for numbers above 50_000_000
fn pollards_rho(n: u64) -> Option<u64> {
    if n > 67_108_863 {
        return _pollards_rho_par(n);
    }
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
            return Some(d as u64);
        }
    }
    None
}

fn _pollards_rho_par(n: u64) -> Option<u64> {
    let n = u128::from(n);
    (2..(n - 2)).into_par_iter().find_map_any(|s| {
        let mut x = s;
        let mut y = s;
        let mut d = 1;
        while d == 1 {
            x = ((x * x) + 1) % n;
            y = ((y * y) + 1) % n;
            y = ((y * y) + 1) % n;
            d = x.abs_diff(y).gcd(&n);
        }
        if d != n { Some(d as u64) } else { None }
    })
}

// Factor out all primes up to 37 and put them into the map. Then apply the 64-bit Miller-Rabin test to the result.
/// Catch all "easy" to find factors.
pub fn partial_factorization(mut n: u64, prime_factors: &mut BTreeMap<u64, u64>) -> u64 {
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
            prime_factors.insert(p, ctr);
        }
    }

    // miller_rabin(n) will not catch this
    if n == 1 {
        return n;
    }

    if is_prime_partial(n) {
        prime_factors.insert(n, 1);
        n = 1;
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
    let mut prime_factors = BTreeMap::new();

    // Handle numbers with easy to find divisors.
    // This catches anything divisible by a small prime and anything with a factor that the Miller-Rabin test can find.
    n = partial_factorization(n, &mut prime_factors);
    if n == 1 {
        return prime_factors.into_iter().collect_vec();
    }

    // Iteratively use Pollard's Rho and Miller-Rabin on the divisors, splitting them until primes are found
    let mut divisors = vec![n];
    while !divisors.is_empty() {
        let d = divisors.pop().unwrap();
        match miller_rabin(d) {
            super::miller_rabin::MRTest::Prime => {
                prime_factors.entry(d).and_modify(|x| *x += 1).or_insert(1);
                continue;
            }
            super::miller_rabin::MRTest::Composite(w) => match w {
                Some(x) => {
                    divisors.push(x);
                    divisors.push(d / x);
                    continue;
                }
                None => (), // go on to Pollard's Rho,
            },
        }
        if let Some(x) = pollards_rho(d) {
            divisors.push(x);
            divisors.push(d / x);
        } else {
            break;
        }
    }

    prime_factors.into_iter().collect_vec()
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

#[test]
#[ignore = "visualization"]
fn speed_tests() {
    for n in [5_392_920_426, 7_769_341_109] {
        let start = std::time::Instant::now();
        prime_factorization(n);
        let el = start.elapsed();
        println!("factored {n} in {:?}", el);
    }
}
