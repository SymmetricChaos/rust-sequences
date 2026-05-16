use crate::utils::divisibility::prime_factorization;

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

/// Jordan's totient function.
/// Defined as 0 for n == 0.
pub fn jordan_totient(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    todo!()
}
