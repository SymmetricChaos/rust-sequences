use crate::core::traits::Increment;
use crate::utils::divisibility::{prime_factorization, prime_signature};
use num::{BigInt, CheckedAdd, CheckedMul, Integer, One};
use std::{
    collections::{BinaryHeap, HashMap},
    hash::Hash, // Found to be much faster than BTreeMap
};

/// The prime natural numbers, those having exactly two distinct factors.
///
/// 2, 3, 5, 7, 11, 13, 17, 19, 23, 29...
pub struct Primes<T> {
    sieve: HashMap<T, Vec<T>>,
    n: T,
}

impl<T: CheckedAdd + Integer + Hash + Clone> Primes<T> {
    pub fn new() -> Self {
        Self {
            sieve: HashMap::<T, Vec<T>>::new(),
            n: T::one(),
        }
    }
}

impl Primes<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: CheckedAdd + Hash + Integer + Clone> Iterator for Primes<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.n.incr()?;
            if !self.sieve.contains_key(&self.n) {
                let v = self.n.checked_add(&self.n)?;
                self.sieve.insert(v, vec![self.n.clone()]);
                return Some(self.n.clone());
            } else {
                let factors = &self.sieve[&self.n].clone();
                for factor in factors {
                    let v = factor.checked_add(&self.n)?;
                    if self.sieve.contains_key(&v) {
                        self.sieve.get_mut(&v).unwrap().push(factor.clone());
                    } else {
                        self.sieve.insert(v, vec![factor.clone()]);
                    }
                }
                self.sieve.remove(&self.n);
            }
        }
    }
}

/// The the prime factorization of each positive integer.
/// For instance 20 = 2^2 + 5^1 and is written here as [(2,2), (5,1)]
/// [], [(2, 1)], [(3, 1)], [(2, 2)], [(5, 1)], [(2, 1), (3, 1)], [(7, 1)], [(2, 3)], [(3, 2)], [(2, 1), (5, 1)]
pub struct PrimeFactorizations {
    ctr: u64,
}

impl PrimeFactorizations {
    /// Only u64 output is supported
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for PrimeFactorizations {
    type Item = Vec<(u64, u64)>;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr = self.ctr.checked_add(1)?;

        Some(prime_factorization(self.ctr))
    }
}

/// The greatest prime factor of each positive integer. Defined as 1 for 1.
/// 1, 2, 3, 2, 5, 3, 7, 2, 3, 5, 11, 3, 13, 7
pub struct GreatestPrimeFactor {
    ctr: u64,
}

impl GreatestPrimeFactor {
    /// Only u64 output is supported
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for GreatestPrimeFactor {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr = self.ctr.checked_add(1)?;

        if self.ctr.is_one() {
            return Some(1);
        }

        Some(prime_factorization(self.ctr).last()?.0)
    }
}

/// The least prime factor of each positive integer. Defined as 1 for 1.
/// 1, 2, 3, 2, 5, 2, 7, 2, 3, 2, 11, 2, 13, 2
pub struct LeastPrimeFactor {
    ctr: u64,
}

impl LeastPrimeFactor {
    /// Only u64 output is supported
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for LeastPrimeFactor {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr = self.ctr.checked_add(1)?;

        if self.ctr.is_one() {
            return Some(1);
        }

        Some(prime_factorization(self.ctr)[0].0)
    }
}

/// The the prime signature of each positive integer. The powers of the prime factorization in decreasing order.
/// For instance the prime signature of 3918213 is [4, 2,1] because 3918213 = 3^4 * 13^1 * 61^2.
/// [], [1], [1], [2], [1], [1, 1], [1], [3], [2], [1, 1]...
pub struct PrimeSignatures {
    ctr: u64,
}

impl PrimeSignatures {
    /// Only u64 output is supported
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for PrimeSignatures {
    type Item = Vec<u64>;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr = self.ctr.checked_add(1)?;

        Some(prime_signature(self.ctr))
    }
}

// Representation of a prime power for use in the generator
#[derive(Eq, PartialEq)]
struct PrimePower<T: Eq + PartialEq> {
    value: T,
    prime: T,
}

impl<T: Clone + CheckedMul + Eq + PartialEq> PrimePower<T> {
    fn new(p: T) -> Self {
        Self {
            value: p.clone(),
            prime: p,
        }
    }

    fn next(mut self) -> Option<Self> {
        self.value = self.value.checked_mul(&self.prime)?;
        Some(self)
    }

    fn is_prime(&self) -> bool {
        self.value == self.prime
    }
}

// Inverted ordering
impl<T: Eq + PartialEq + PartialOrd + Ord> Ord for PrimePower<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.value.cmp(&self.value)
    }
}

// Inverted ordering
impl<T: Eq + PartialEq + PartialOrd + Ord> PartialOrd for PrimePower<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.value.partial_cmp(&self.value)
    }
}

/// The perfect powers of primes.
/// 1, 2, 3, 4, 5, 7, 8, 9, 11, 13, 16, 17, 19, 23, 25, 27, 29, 31, 32, 37...
pub struct PrimePowers<T: Eq> {
    priority_queue: BinaryHeap<PrimePower<T>>,
    primes: Primes<T>,
}

impl<T: CheckedAdd + CheckedMul + Clone + Hash + Ord + Integer> PrimePowers<T> {
    pub fn new() -> Self {
        Self {
            priority_queue: BinaryHeap::new(),
            primes: Primes::new(),
        }
    }
}

impl PrimePowers<BigInt> {
    pub fn new_big() -> Self {
        Self {
            priority_queue: BinaryHeap::new(),
            primes: Primes::new_big(),
        }
    }
}

impl<T: CheckedAdd + CheckedMul + Clone + Hash + Integer> Iterator for PrimePowers<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.priority_queue.is_empty() {
            self.priority_queue
                .push(PrimePower::new(self.primes.next()?));
            return Some(T::one());
        } else {
            let smallest = self.priority_queue.pop()?;
            let out = smallest.value.clone();
            if smallest.is_prime() {
                self.priority_queue
                    .push(PrimePower::new(self.primes.next()?));
            }
            self.priority_queue.push(smallest.next()?);
            return Some(out);
        }
    }
}

/// The primorials, the products of the sequential primes.
/// 1, 2, 6, 30, 210, 2310, 30030, 510510, 9699690...
pub struct Primorial<T> {
    prod: T,
    primes: Primes<T>,
}

impl<T: CheckedAdd + CheckedMul + Hash + Integer + Clone> Primorial<T> {
    pub fn new() -> Self {
        Self {
            prod: T::one(),
            primes: Primes::new(),
        }
    }
}

impl Primorial<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: CheckedAdd + CheckedMul + Hash + Integer + Clone> Iterator for Primorial<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.prod.clone();
        self.prod = self.prod.checked_mul(&self.primes.next()?)?;
        Some(out)
    }
}

/// The number of prime factors of each positive integer, counted with multiplicity. The prime Ω function.
pub struct NPrimeFactors {
    ctr: u64,
}

impl NPrimeFactors {
    /// Only u64 output is supported
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for NPrimeFactors {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr = self.ctr.checked_add(1)?;

        Some(prime_factorization(self.ctr).iter().map(|x| x.1).sum())
    }
}

/// The number of distinct prime factors of each positive integer, counted with multiplicity. The prime ω function.
pub struct NDistinctPrimeFactors {
    ctr: u64,
}

impl NDistinctPrimeFactors {
    /// Only u64 output is supported
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for NDistinctPrimeFactors {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr = self.ctr.checked_add(1)?;

        Some(prime_factorization(self.ctr).len() as u64)
    }
}

crate::print_sequences!(
    PrimeSignatures::new(), 30, "{:?}", ", ";
    PrimeFactorizations::new(), 15, "{:?}", ", ";
);

crate::check_iteration_times!(
    Primes::new_big(), 75_000;
    Primes::<u32>::new(), 75_000;
);

crate::check_sequences!(
    Primes::new_big(), skip 0, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    Primes::new_big(), skip 1000, [7927, 7933, 7937, 7949, 7951, 7963, 7993, 8009, 8011, 8017];
    PrimePowers::<u32>::new(), [1, 2, 3, 4, 5, 7, 8, 9, 11, 13, 16, 17, 19, 23, 25, 27, 29, 31, 32, 37];
    Primorial::<u32>::new(), [1, 2, 6, 30, 210];
    GreatestPrimeFactor::new(), [1, 2, 3, 2, 5, 3, 7, 2, 3, 5, 11, 3, 13, 7];
    LeastPrimeFactor::new(), [1, 2, 3, 2, 5, 2, 7, 2, 3, 2, 11, 2, 13, 2];
);
