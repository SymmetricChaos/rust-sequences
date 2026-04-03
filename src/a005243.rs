use num::{BigInt, Integer};
use std::collections::BinaryHeap;

/// Starts with 1 and 2 and then every sum of sequential terms appears.
pub struct A005243<T> {
    terms: Vec<T>,
    heap: BinaryHeap<T>,
}

impl<T: Clone + Integer> A005243<T> {
    pub fn new() -> Self {
        Self {
            terms: vec![T::one(), T::one() + T::one()],
            heap: BinaryHeap::new(),
        }
    }
}

impl A005243<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Clone + Integer> Iterator for A005243<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!("calculate all sums and push them onto the heap with Reverse(n) to get a min-heap");

        let out = self.heap.pop()?;
        self.terms.push(out.clone());
        Some(out)
    }
}
