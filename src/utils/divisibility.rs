use crate::utils::{
    miller_rabin::{is_prime, is_prime_partial, miller_rabin},
    pollard::pollards_rho,
};
use itertools::Itertools;
use num::{CheckedAdd, CheckedMul, Integer, rational::Ratio};
use std::collections::BTreeMap;

/// Factor out all primes up to 37 and put them into the map.
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

    n
}

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
    // This catches anything divisible by a small prime.
    n = partial_factorization(n, &mut prime_factors);
    if n == 1 {
        return prime_factors.into_iter().collect_vec();
    }

    // Use Miller-Rabin
    let mut divisors = match miller_rabin(n) {
        // If the remaining part is prime immediately return to factorization.
        super::miller_rabin::MRTest::Prime => {
            prime_factors.entry(n).and_modify(|x| *x += 1).or_insert(1);
            return prime_factors.into_iter().collect_vec();
        }
        // If it is composite see if the MR test found a factor
        super::miller_rabin::MRTest::Composite(w) => match w {
            // If we did then split the remaining part into divisors
            Some(x) => {
                let mut div = Vec::new();
                div.push(x);
                div.push(n / x);
                div
            }
            // If we didn't just put the remaining part as a divisor
            None => vec![n],
        },
    };

    // Iteratively use Pollard's Rho on the divisors, splitting them until primes are found
    while !divisors.is_empty() {
        let d = divisors.pop().unwrap();
        if is_prime_partial(d) {
            prime_factors.entry(d).and_modify(|x| *x += 1).or_insert(1);
            continue;
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

/// Factor a number into prime powers
pub fn prime_power_factorization(n: u64) -> Vec<u64> {
    prime_factorization(n)
        .iter()
        .map(|x| x.0.pow(x.1 as u32))
        .collect_vec()
}

/// The unique prime divisors of n.
pub fn prime_divisors(n: u64) -> Vec<u64> {
    prime_factorization(n)
        .into_iter()
        .map(|(p, _)| p)
        .collect_vec()
}

/// All of the divisors of n.
/// Defined as [0] for n = 0.
pub fn divisors(n: u64) -> Vec<u64> {
    if n <= 1 {
        return vec![n];
    }

    let mut out = vec![1, n];

    if is_prime(n) {
        return out;
    }

    for f in 2..=n.isqrt() {
        let (d, r) = n.div_rem(&f);

        if r == 0 {
            if f == d {
                out.push(f);
            } else {
                out.push(f);
                out.push(d);
            }
        }
    }

    out.sort();

    out
}

/// All of the divisors of n except itself.
/// Defined as [] for n = 0.
pub fn proper_divisors(n: u64) -> Vec<u64> {
    if n <= 1 {
        return vec![];
    }

    let mut out = vec![1];

    if is_prime(n) {
        return out;
    }

    for f in 2..=n.isqrt() {
        let (d, r) = n.div_rem(&f);
        if r == 0 {
            if f == d {
                out.push(f);
            } else {
                out.push(f);
                out.push(d);
            }
        }
    }

    out.sort();

    out
}

/// The number of prime factors of n counted with multiplicity.
pub fn big_omega(n: u64) -> u64 {
    prime_factorization(n).into_iter().map(|(_, m)| m).sum()
}

/// The number of distinct prime factors of n.
pub fn small_omega(n: u64) -> u64 {
    prime_factorization(n).len() as u64
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

/// Number of divisors of n. Also known as σ_0(n) "sigma sub zero of n".
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

/// Sum of all divisors of n. Also known as σ_1(n) "sigma sub one of n".
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

/// The number theoretic sigma function.
pub fn sigma(n: u64, e: u32) -> Option<u64> {
    if e == 0 {
        Some(number_of_divisors(n))
    } else if e == 1 {
        sum_of_divisors(n)
    } else {
        let divs = divisors(n);
        let mut out = 0;
        for d in divs {
            println!("{d} {}", d.pow(e));
            out = out.checked_add(&d.pow(e))?;
        }
        println!();
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

crate::print_sequences!(
    divisors(2*2*5*7).into_iter(), 20, "{:?}", ", ";
    prime_factorization(363747780).into_iter(), 10, "{:?}", ", ";
    prime_power_factorization(363747780).into_iter(), 10, "{:?}", ", ";
);

#[test]
#[ignore = "visualization"]
fn speed_tests() {
    for n in [125_047_306, 5_392_920_426, 6_031_036_133, 7_769_341_109] {
        let start = std::time::Instant::now();
        prime_factorization(n);
        let el = start.elapsed();
        println!("factored {n} in {:?}", el);
    }
}
