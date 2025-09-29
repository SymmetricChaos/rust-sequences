use num::{BigInt, CheckedAdd, CheckedMul, One, PrimInt, Zero};
use std::{
    collections::{BinaryHeap, HashMap},
    hash::Hash, // Found to be much faster than BTreeMap
};

/// The prime natural numbers.
/// 2, 3, 5, 7, 11, 13, 17, 19, 23, 29...
pub struct Primes<T> {
    sieve: HashMap<T, Vec<T>>,
    n: T,
}

impl<T: PrimInt> Primes<T> {
    pub fn new() -> Self {
        Self {
            sieve: HashMap::<T, Vec<T>>::new(),
            n: T::one(),
        }
    }
}

impl Primes<BigInt> {
    pub fn new_big() -> Self {
        Self {
            sieve: HashMap::<BigInt, Vec<BigInt>>::new(),
            n: BigInt::one(),
        }
    }
}

impl<T: Zero + One + CheckedAdd + Clone + Hash + Eq> Iterator for Primes<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.n = self.n.checked_add(&T::one())?;
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

/// The perfect powers of primes.
/// 1, 2, 3, 4, 5, 7, 8, 9, 11, 13, 16, 17, 19, 23, 25, 27, 29, 31, 32, 37...
#[derive(Eq, PartialEq)]
struct PrimePower<T: Eq + PartialEq> {
    value: T,
    prime: T,
}

impl<T: Clone + CheckedMul + Eq + PartialEq> PrimePower<T> {
    pub fn new(p: T) -> Self {
        Self {
            value: p.clone(),
            prime: p,
        }
    }

    pub fn next(mut self) -> Option<Self> {
        self.value = self.value.checked_mul(&self.prime)?;
        Some(self)
    }

    pub fn is_prime(&self) -> bool {
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

pub struct PrimePowers<T: Eq> {
    queue: BinaryHeap<PrimePower<T>>,
    primes: Primes<T>,
}

impl<T: PrimInt> PrimePowers<T> {
    pub fn new() -> Self {
        Self {
            queue: BinaryHeap::new(),
            primes: Primes::new(),
        }
    }
}

impl PrimePowers<BigInt> {
    pub fn new_big() -> Self {
        Self {
            queue: BinaryHeap::new(),
            primes: Primes::new_big(),
        }
    }
}

impl<T: Eq + CheckedAdd + CheckedMul + Clone + Hash + One + Zero + Ord + PartialOrd> Iterator
    for PrimePowers<T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.is_empty() {
            self.queue.push(PrimePower::new(self.primes.next()?));
            return Some(T::one());
        } else {
            let smallest = self.queue.pop()?;
            let out = smallest.value.clone();
            if smallest.is_prime() {
                self.queue.push(PrimePower::new(self.primes.next()?));
            }
            self.queue.push(smallest.next()?);
            return Some(out);
        }
    }
}

crate::check_iteration_times!(
    Primes::new_big(), 21_000;
    Primes::<u32>::new(), 21_000;
);

crate::check_sequences!(
    Primes::new_big(), 0, 10, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    Primes::new_big(), 1000, 10, [7927, 7933, 7937, 7949, 7951, 7963, 7993, 8009, 8011, 8017];
    PrimePowers::<u32>::new(), 0, 20, [1, 2, 3, 4, 5, 7, 8, 9, 11, 13, 16, 17, 19, 23, 25, 27, 29, 31, 32, 37];
);
