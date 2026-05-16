use crate::utils::divisibility::prime_factorization;

/// Euler's totient function. Number of positive integers coprime to n and less than n.
/// Defined as 0 for n == 0.
pub fn totient(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    prime_factorization(n)
        .iter()
        .fold(1, |acc, (p, e)| acc * (p.pow((e - 1) as u32) * (p - 1)))
}

/// Euler's cototient function. Number of positive integers not coprime to n and less than n.
/// Defined as 0 for n == 0.
pub fn cototient(n: u64) -> u64 {
    n - totient(n)
}

/// Jordan's totient function. The number of k-tuples of positive integers that can be made such that the k-tuple is setwise coprime.
/// Defined as 0 for n == 0.
pub fn jordan_totient(n: u64, k: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    prime_factorization(n).iter().fold(1, |acc, (p, e)| {
        acc * (p.pow(((e - 1) as u32) * k) * (p.pow(k) - 1))
    })
}
