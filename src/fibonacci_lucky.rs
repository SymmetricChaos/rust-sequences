use num::{BigInt, CheckedAdd, Integer};

/// The Fibonacci-Lucky numbers. Numbers of the sieve are selected using sequential sum of terms starting with a(0) = 1 and a(1) = 2.
///
/// 1, 2, 4, 5, 7, 10, 11, 13, 16, 19, 20...
pub struct FibonacciLucky<T> {
    idx: T,
    sieve: Vec<[T; 2]>,
}

impl<T: CheckedAdd + Clone + Integer> FibonacciLucky<T> {
    pub fn new() -> Self {
        Self {
            idx: T::one(),
            sieve: Vec::new(),
        }
    }
}

impl FibonacciLucky<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: CheckedAdd + Clone + Integer> Iterator for FibonacciLucky<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

crate::check_sequences!(
    FibonacciLucky::<u32>::new(), [1, 2, 4, 5, 7, 10, 11, 13, 16, 19, 20, 23, 25, 28, 29, 32, 37, 38, 40, 41, 49, 50, 52, 56, 58, 59, 61, 65, 68, 74, 76, 77, 82, 83, 86, 88, 91, 97, 101, 103, 104, 106, 115, 118, 121, 122, 124, 130, 131, 133, 136, 137, 149, 151, 154, 155, 158, 163, 164, 166, 173, 175];
);
