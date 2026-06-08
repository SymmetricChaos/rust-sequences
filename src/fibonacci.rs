use crate::Number;
use num::{BigInt, CheckedAdd, Integer, One, Zero};
use std::collections::VecDeque;

/// The Fibonacci numbers. Starting with 0 and 1 every term is the sum of the two previous.
///
/// ```text
/// 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987...
/// ```
pub struct Fibonacci<T> {
    a: T,
    b: T,
}

impl Fibonacci<Number> {
    pub fn new() -> Self {
        Self { a: 0, b: 1 }
    }
}

#[cfg(feature = "big_int")]
impl Fibonacci<BigInt> {
    pub fn new_big() -> Self {
        Self {
            a: BigInt::zero(),
            b: BigInt::one(),
        }
    }
}

impl<T: Clone + Integer + CheckedAdd> Iterator for Fibonacci<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let t = self.a.checked_add(&self.b)?;
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

/// The bits of the infinite Fibonacci word.
/// 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1...
pub struct FibonacciWord<T> {
    word: VecDeque<bool>,
    zero: T,
    one: T,
}

impl FibonacciWord<Number> {
    /// Note that an internal VecDeque grows at a linear rate as the iterator runs.
    /// If a known number of bits are needed first_n is much more memory efficient.
    pub fn new() -> Self {
        Self {
            word: VecDeque::from(vec![false]),
            zero: 0,
            one: 1,
        }
    }

    /// First n bits of the infinite Fibonacci word.
    /// Panics if n is less than one.
    pub fn first_n(n: i64) -> Vec<Number> {
        assert!(n > 0);
        let mut word = Vec::with_capacity(n as usize + 1);
        word.push(0);
        let mut idx = 0;
        while word.len() < n as usize {
            if word[idx].is_one() {
                word.push(0);
            } else {
                word.push(1);
                word.push(0);
            }
            idx += 1;
        }
        word.truncate(n as usize);
        word
    }
}

#[cfg(feature = "big_int")]
impl FibonacciWord<BigInt> {
    /// Note that an internal VecDeque grows at a linear rate as the iterator runs.
    /// If a known number of bits are needed first_n is much more memory efficient.
    pub fn new_big() -> Self {
        Self {
            word: VecDeque::from(vec![false]),
            zero: BigInt::zero(),
            one: BigInt::one(),
        }
    }

    /// First n bits of the infinite Fibonacci word.
    /// Panics if n is less than one.
    pub fn first_n_big(n: usize) -> Vec<BigInt> {
        let mut word = Vec::with_capacity(n + 1);
        word.push(BigInt::zero());
        let mut idx = 0;
        while word.len() < n as usize {
            if word[idx].is_one() {
                word.push(BigInt::zero());
            } else {
                word.push(BigInt::one());
                word.push(BigInt::zero());
            }
            idx += 1;
        }
        word.truncate(n);
        word
    }
}

impl<T: Clone> Iterator for FibonacciWord<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.word.pop_front()?;
        if cur {
            self.word.push_back(false);
            Some(self.one.clone())
        } else {
            self.word.push_back(true);
            self.word.push_back(false);
            Some(self.zero.clone())
        }
    }
}

/// The sucessive Fibonacci words as strings.
/// 0, 01, 010, 01001, 01001010, 0100101001001, 010010100100101001010...
pub struct FibonacciStrings {
    a: String,
    b: String,
}

impl FibonacciStrings {
    pub fn new() -> Self {
        Self {
            a: String::from("0"),
            b: String::from("01"),
        }
    }
}

impl Iterator for FibonacciStrings {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let out = Some(self.a.clone());
        let mut t = self.b.clone();
        t.push_str(&self.a);
        self.a = self.b.clone();
        self.b = t;
        out
    }
}

crate::check_iteration_times!(
    Fibonacci::new_big(), 160_000;
);

crate::check_sequences!(
    Fibonacci::new_big(), [0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
    FibonacciWord::new_big(), [0, 1, 0, 0, 1, 0, 1, 0, 0, 1];
);

crate::print_sequences!(
    FibonacciStrings::new(), 7;
);

crate::sample_sequences!(
    Fibonacci::new();
    FibonacciWord::new();
    FibonacciStrings::new();
);
