use num::{BigInt, CheckedAdd, Integer};
use std::collections::HashMap;
use std::hash::Hash;

pub struct EratosthenesArray<T> {
    sieve: HashMap<T, Vec<T>>,
    n: T,
}

impl<T: CheckedAdd + Integer + Hash + Clone> EratosthenesArray<T> {
    pub fn new() -> Self {
        Self {
            sieve: HashMap::<T, Vec<T>>::new(),
            n: T::one(),
        }
    }
}

impl EratosthenesArray<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: CheckedAdd + Hash + Integer + Clone> Iterator for EratosthenesArray<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
