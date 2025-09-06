use std::collections::VecDeque;

use num::{BigInt, One, Zero};

/// The Fibonacci numbers.
/// 0, 1, 1, 2, 3, 5, 8, 13, 21, 34...
pub struct Fibonacci {
    a: BigInt,
    b: BigInt,
}

impl Fibonacci {
    pub fn new() -> Self {
        Self {
            a: BigInt::from(0),
            b: BigInt::from(1),
        }
    }
}

impl Iterator for Fibonacci {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let t = &self.a + &self.b;
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

/// The bits of the infinite Fibonacci word.
/// 0, 1, 0, 0, 1, 0, 1, 0, 0, 1...

pub struct FibonacciWord<T> {
    word: VecDeque<bool>,
    zero: T,
    one: T,
}

impl<T: One + Zero + PartialEq> FibonacciWord<T> {
    /// Note that an internal VecDeque grows at a linear rate as the iterator runs.
    /// If a known number of bits are needed first_n is much more memory efficient.
    pub fn new() -> Self {
        Self {
            word: VecDeque::from(vec![false]),
            zero: T::zero(),
            one: T::one(),
        }
    }

    /// First n bits of the infinite Fibonacci word.
    /// Panics if n is less than one.
    pub fn first_n(n: i64) -> Vec<T> {
        assert!(n > 0);
        let mut word = Vec::with_capacity(n as usize + 1);
        word.push(T::zero());
        let mut idx = 0;
        while word.len() < n as usize {
            if word[idx].is_one() {
                word.push(T::zero());
            } else {
                word.push(T::one());
                word.push(T::zero());
            }
            idx += 1;
        }
        word.truncate(n as usize);
        word
    }
}

impl<T: Clone> Iterator for FibonacciWord<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.word.pop_front()?;
        println!("{}", self.word.len());
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
/// 0, 01, 010, 01001, 01001010, 0100101001001...
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

crate::check_sequences!(
    Fibonacci::new(), 0, 10, [0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
    FibonacciWord::<u8>::new(), 0, 10, [0, 1, 0, 0, 1, 0, 1, 0, 0, 1];
);

crate::print_a_few!(
    FibonacciStrings::new(), 0, 7;
);
