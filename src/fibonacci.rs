use num::BigInt;

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
        let mut t = self.a.clone();
        t.push_str(&self.b);
        self.a = self.b.clone();
        self.b = t;
        out
    }
}

crate::check_sequences!(
    Fibonacci::new(), 0, 10, [0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
);
