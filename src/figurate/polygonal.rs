use num::{BigInt, One, Zero};

/// The polygonal numbers with selectable order.
pub struct Polygonal {
    val: BigInt,
    gnomon: BigInt,
    order: BigInt,
}

impl Polygonal {
    /// The order, k, is the change in the size of the gnomon at each step.
    /// k = 0 prodces the natural numbers
    /// k = 1 produces the triangular numbers
    /// k = 2 produces the square numbers
    /// and so on for higher orders
    /// Negative values of k are allowed but do not have standard names.
    pub fn new<T>(k: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            val: BigInt::zero(),
            gnomon: BigInt::one(),
            order: BigInt::from(k),
        }
    }

    /// The order, k, is the change in the size of the gnomon at each step.
    /// k = 0 prodces the natural numbers
    /// k = 1 produces the triangular numbers
    /// k = 2 produces the square numbers
    /// and so on for higher orders
    /// Negative values of k are allowed but do not have standard names.
    /// Negative values of n are generalized polygonal numbers.
    pub fn nth<T>(n: T, k: T) -> BigInt
    where
        BigInt: From<T>,
    {
        let k = &BigInt::from(k);
        let n = &BigInt::from(n);
        (k * n * n - (k - 2) * n) / 2
    }
}

impl Iterator for Polygonal {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val += &self.gnomon;
        self.gnomon += &self.order;
        Some(out)
    }

    // Optimized .nth() makes .skip() quicker
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.gnomon += &self.order * n;

        let n = &((&self.gnomon - 1) / &self.order);
        let k = &self.order;
        self.val = (k * n * n - (k - 2) * n) / 2;

        let out = self.val.clone();

        self.val += &self.gnomon;
        self.gnomon += &self.order;

        Some(out)
    }
}

// For testing the .nth() words
crate::print_values!(
    Polygonal::new(1), 0, 10;
    Polygonal::new(1), 1, 10;
    Polygonal::new(1), 2, 10;
    Polygonal::new(1), 3, 10;
    Polygonal::new(1), 4, 10;

    Polygonal::new(2), 0, 10;
    Polygonal::new(2), 1, 10;
    Polygonal::new(2), 2, 10;
    Polygonal::new(2), 3, 10;
    Polygonal::new(2), 4, 10;
);

crate::check_sequences!(
    Polygonal::new(0), 0, 10, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    Polygonal::new(1), 0, 10, [0, 1, 3, 6, 10, 15, 21, 28, 36, 45];
    Polygonal::new(2), 0, 10, [0, 1, 4, 9, 16, 25, 36, 49, 64, 81];
    Polygonal::new(3), 0, 10, [0, 1, 5, 12, 22, 35, 51, 70, 92, 117];
);

#[test]
fn test_nth() {
    for i in 0..10 {
        print!("{}, ", Polygonal::nth(i, 1))
    }
    println!("");
    for i in 0..10 {
        print!("{}, ", Polygonal::nth(i, 2))
    }
}
