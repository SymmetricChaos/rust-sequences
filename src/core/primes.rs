use crate::core::traits::Increment;
use num::{BigInt, CheckedAdd, CheckedMul, Integer};
use std::{
    collections::{BinaryHeap, HashMap},
    hash::Hash, // Found to be much faster than BTreeMap
};

/// The prime natural numbers, those having exactly two distinct divisors.
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
///
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
        Self::new()
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

/// The primorials, the partial products of the primes.
///
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
        self.prod = match self.prod.checked_mul(&self.primes.next()?) {
            Some(n) => n,
            None => return Some(out),
        };
        Some(out)
    }
}

crate::check_iteration_times!(
    Primes::new_big(), 75_000;
    Primes::<u32>::new(), 75_000;
);

crate::check_sequences!(
    Primes::new_big(),         [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271];
    PrimePowers::<u32>::new(), [1, 2, 3, 4, 5, 7, 8, 9, 11, 13, 16, 17, 19, 23, 25, 27, 29, 31, 32, 37, 41, 43, 47, 49, 53, 59, 61, 64, 67, 71, 73, 79, 81, 83, 89, 97, 101, 103, 107, 109, 113, 121, 125, 127, 128, 131, 137, 139, 149, 151, 157, 163, 167, 169, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227];
    Primorial::<u64>::new(),   [1_u64, 2, 6, 30, 210, 2310, 30030, 510510, 9699690, 223092870, 6469693230, 200560490130, 7420738134810, 304250263527210, 13082761331670030, 614889782588491410];
);
