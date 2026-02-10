use crate::core::prime::Primes;
use itertools::Itertools;
use num::{BigInt, CheckedAdd, CheckedDiv, Integer, One, Signed, Zero};
use std::cmp::Reverse;
use std::{
    collections::{BinaryHeap, HashSet},
    hash::Hash,
};

/// The smooth numbers, those natural numbers for which the only prime divisors are less than or equal to n.
pub struct Smooth<T> {
    ctr: T,
    primes: Vec<T>,
}

impl<T: CheckedAdd + CheckedDiv + Clone + Hash + Integer> Smooth<T> {
    /// Panics if n is less than two.
    /// If n is very large initializing the set of primes may impose an extreme time and memory burden. There are more than two hundred million primes less than u32::MAX.
    pub fn new(n: T) -> Self {
        assert!(n > T::one());
        Self {
            ctr: T::zero(),
            primes: Primes::new().take_while(|x| *x <= n).collect_vec(),
        }
    }
}

impl Smooth<BigInt> {
    /// Panics if n is less than two.
    /// If n is very large initializing the set of primes may impose an extreme time and memory burden. There are more than two hundred million primes less than u32::MAX.
    pub fn new_big<N>(n: N) -> Self
    where
        BigInt: From<N>,
    {
        let n = BigInt::from(n);
        assert!(n.is_positive());
        Self {
            ctr: BigInt::zero(),
            primes: Primes::new_big().take_while(|x| *x <= n).collect_vec(),
        }
    }
}

impl<T: CheckedAdd + CheckedDiv + Clone + Integer> Iterator for Smooth<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.ctr = self.ctr.checked_add(&T::one())?;
            let mut n = self.ctr.clone();
            for p in self.primes.iter() {
                while n.is_multiple_of(p) {
                    n = n.checked_div(&p)?; // this always succeeds but is the only way I could find to allow division without cloning p
                }
            }
            if n.is_one() {
                return Some(self.ctr.clone());
            }
        }
    }
}

/// The regular numbers, those which have only the prime divisors 2, 3, and 5. Thousands of times faster than Smooth::new(5) but uses increasingly more memory as it generates terms.
/// 1, 2, 3, 4, 5, 6, 8, 9, 10, 12, 15, 16...
pub struct Regular<T> {
    heap: BinaryHeap<Reverse<T>>,
    set: HashSet<T>,
}

impl<T: CheckedAdd + One + Ord> Regular<T> {
    pub fn new() -> Self {
        let mut heap = BinaryHeap::new();
        heap.push(Reverse(T::one()));
        Self {
            heap,
            set: HashSet::new(),
        }
    }
}

impl Regular<BigInt> {
    pub fn new_big() -> Self {
        let mut heap = BinaryHeap::new();
        heap.push(Reverse(BigInt::one()));
        Self {
            heap,
            set: HashSet::new(),
        }
    }
}

impl<T: CheckedAdd + Clone + Ord + Hash> Iterator for Regular<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let h = self.heap.pop()?.0;

        // Save some space
        self.set.remove(&h);

        // Calculate 2h and short circuit if the values is too large or insert if it is new
        let mut a = h.checked_add(&h);
        match a.clone() {
            Some(n) => {
                if !self.set.iter().contains(&n) {
                    self.set.insert(n.clone());
                    self.heap.push(Reverse(n));
                }
            }
            None => return Some(h),
        }

        // Same for 3h
        a = a.unwrap().checked_add(&h);
        match a.clone() {
            Some(n) => {
                if !self.set.iter().contains(&n) {
                    self.set.insert(n.clone());
                    self.heap.push(Reverse(n));
                }
            }
            None => return Some(h),
        }

        // Same for 5h but with a short circuit if 4h overflows
        match a.unwrap().checked_add(&h) {
            Some(b) => match b.checked_add(&h) {
                Some(n) => {
                    if !self.set.iter().contains(&n) {
                        self.set.insert(n.clone());
                        self.heap.push(Reverse(n));
                    }
                }
                None => return Some(h),
            },
            None => return Some(h),
        }

        Some(h)
    }
}

crate::check_iteration_times!(
    Smooth::<i32>::new(5), 1690;
    Regular::<i32>::new(), 1690; // Only 1690 values are possible for i32
    Regular::<u32>::new(), 1690; // this takes longer because it has more space and doesn't short circuit yet
);

crate::check_sequences!(
    Smooth::new_big(5), 0, 20, [1, 2, 3, 4, 5, 6, 8, 9, 10, 12, 15, 16, 18, 20, 24, 25, 27, 30, 32, 36];
    Regular::<i32>::new(), 0, 20, [1, 2, 3, 4, 5, 6, 8, 9, 10, 12, 15, 16, 18, 20, 24, 25, 27, 30, 32, 36];
);
