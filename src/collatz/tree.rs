use num::{BigInt, One};

/// The Collatz tree. Each layer contains the numbers that have a trajectory one greater than the pevious layer.
///
/// [1], [2], [4], [8], [16], [5, 32], [10, 64], [3, 20, 21, 128]...
pub struct CollatzTree {
    layer: Vec<BigInt>,
}

impl CollatzTree {
    pub fn new() -> Self {
        Self {
            layer: vec![BigInt::one()],
        }
    }
}

impl Iterator for CollatzTree {
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
        self.layer.sort();
        Some(out)
    }
}

crate::check_sequences!(
    CollatzTree::new().flatten(), [1, 2, 4, 8, 16, 5, 32, 10, 64, 3, 20, 21, 128, 6, 40, 42, 256, 12, 13, 80, 84, 85, 512, 24, 26, 160, 168, 170, 1024, 48, 52, 53, 320, 336, 340, 341, 2048, 17, 96, 104, 106, 113, 640, 672, 680, 682, 4096, 34, 35, 192, 208, 212, 213, 226, 227, 1280, 1344, 1360, 1364];
);
