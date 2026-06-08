use num::{BigInt, One};

use crate::Number;

/// The Collatz tree. Each layer contains the numbers that have a trajectory one step longer than the pevious layer. When flattened a permutation of the positive integers.
///
/// ```text
/// [1], [2], [4], [8], [16], [5, 32], [10, 64], [3, 20, 21, 128]...
///
/// flattened
/// 1, 2, 4, 8, 16, 5, 32, 10, 64, 3, 20, 21, 128, 6, 40, 42, 256, 12...
/// ```
pub struct CollatzTree<T> {
    layer: Vec<T>,
}

impl CollatzTree<Number> {
    pub fn new() -> Self {
        Self { layer: vec![1] }
    }
}

#[cfg(feature = "big_int")]
impl CollatzTree<BigInt> {
    pub fn new_big() -> Self {
        Self {
            layer: vec![BigInt::one()],
        }
    }
}

impl Iterator for CollatzTree<Number> {
    type Item = Vec<Number>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.layer.clone();
        self.layer.clear();
        for n in out.iter() {
            self.layer.push(n * 2);
            if n % 6 == 4 && *n != 4 {
                self.layer.push((n - 1) / 3);
            }
        }
        self.layer.sort(); // does this need to also check for uniqueness at some point?
        Some(out)
    }
}

#[cfg(feature = "big_int")]
impl Iterator for CollatzTree<BigInt> {
    type Item = Vec<BigInt>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.layer.clone();
        self.layer.clear();
        for n in out.iter() {
            self.layer.push(n * 2);
            if n % 6 == BigInt::from(4) && *n != BigInt::from(4) {
                self.layer.push((n - 1) / 3);
            }
        }
        self.layer.sort(); // does this need to also check for uniqueness at some point?
        Some(out)
    }
}

crate::check_sequences!(
    CollatzTree::new().flatten(), [1, 2, 4, 8, 16, 5, 32, 10, 64, 3, 20, 21, 128, 6, 40, 42, 256, 12, 13, 80, 84, 85, 512, 24, 26, 160, 168, 170, 1024, 48, 52, 53, 320, 336, 340, 341, 2048, 17, 96, 104, 106, 113, 640, 672, 680, 682, 4096, 34, 35, 192, 208, 212, 213, 226, 227, 1280, 1344, 1360, 1364];
);

crate::sample_sequences!(
    CollatzTree::new().map(|x| format!("{:?}",x));
    CollatzTree::new().flatten();
);
