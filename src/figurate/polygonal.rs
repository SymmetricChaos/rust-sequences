use num::{BigInt, One, Zero};

/// The polygonal numbers with selectable order.
pub struct Polygonal {
    val: BigInt,
    gnomon: BigInt,
    order: BigInt,
}

impl Polygonal {
    /// The order, k, is the number of sides of the polygon.
    /// k = 2 produces the natural numbers
    /// k = 3 produces the triangular numbers
    /// k = 4 produces the square numbers
    /// and so on for higher orders
    /// Lower values of k are allowed but do not have standard names.
    pub fn new_big<T>(k: T) -> Self
    where
        BigInt: From<T>,
    {
        Self {
            val: BigInt::zero(),
            gnomon: BigInt::one(),
            order: BigInt::from(k) - 2,
        }
    }

    /// The order, k, is the number of sides of the polygon.
    /// k = 2 produces the natural numbers
    /// k = 3 produces the triangular numbers
    /// k = 4 produces the square numbers
    /// and so on for higher orders
    /// Lower values of k are allowed but do not have standard names.
    pub fn nth<T>(n: T, k: T) -> BigInt
    where
        BigInt: From<T>,
    {
        let k = &BigInt::from(k);
        let n = &BigInt::from(n);
        ((k - 2) * n * n - (k - 4) * n) / 2
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

        let idx = if self.order.is_zero() {
            &self.gnomon
        } else {
            &((&self.gnomon - 1) / &self.order)
        };

        let k = &self.order;
        self.val = (k * idx * idx - (k - 2) * idx) / 2;

        let out = self.val.clone();

        self.val += &self.gnomon;
        self.gnomon += &self.order;

        Some(out)
    }
}

crate::print_sequences!(
    // For testing that .nth() words
    Polygonal::new_big(3), 0, 10;
    Polygonal::new_big(3), 1, 10;
    Polygonal::new_big(3), 2, 10;
    Polygonal::new_big(3), 3, 10;
    Polygonal::new_big(3), 4, 10;

    Polygonal::new_big(4), 0, 10;
    Polygonal::new_big(4), 1, 10;
    Polygonal::new_big(4), 2, 10;
    Polygonal::new_big(4), 3, 10;
    Polygonal::new_big(4), 4, 10;

);

crate::check_sequences!(
    Polygonal::new_big(2), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    Polygonal::new_big(3), [0, 1, 3, 6, 10, 15, 21, 28, 36, 45];
);

#[test]
fn test_nth() {
    for i in 0..10 {
        print!("{}, ", Polygonal::nth(i, 2))
    }
    println!("");
    for i in 0..10 {
        print!("{}, ", Polygonal::nth(i, 3))
    }
}
